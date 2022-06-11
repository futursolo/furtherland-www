use ahash::AHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use futures::future::FutureExt;
use futures::future::Shared;
use futures::lock::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlScriptElement};

use crate::client::Client;
use crate::error::InternalResult;
use crate::request::{Method, Request};
use crate::response::{BaseResponse, SerdeResponse};

use super::*;

type RequestMap = HashMap<u64, Shared<LocalBoxFuture<'static, InternalResult<BaseResponse>>>>;

#[derive(Debug)]
pub struct CacheExchange {
    requests: Rc<Mutex<RequestMap>>,
    completed_requests: Rc<Mutex<HashMap<u64, BaseResponse>>>,
}

impl Default for CacheExchange {
    fn default() -> Self {
        let document = window().and_then(|m| m.document()).unwrap_throw();

        if let Some(m) = document.head() {
            let m: &Element = m.as_ref();

            if let Some(m) = m
                .query_selector("#yew-query-request-cache")
                .ok()
                .and_then(|m| m)
                .and_then(|m| m.dyn_into::<HtmlScriptElement>().ok())
                .and_then(|m| m.text().ok())
            {
                if let Ok(m) = serde_json::from_str::<HashMap<u64, SerdeResponse>>(&m) {
                    let mut requests = HashMap::new();

                    for (k, v) in m.into_iter() {
                        let v = BaseResponse::from(v.to_owned());
                        let fur = Box::pin(futures::future::ok(v))
                            as LocalBoxFuture<'static, InternalResult<BaseResponse>>;

                        requests.insert(k, fur.shared());
                    }

                    return Self {
                        requests: Rc::new(Mutex::new(requests)),
                        completed_requests: Default::default(),
                    };
                }
            }
        }

        Self {
            requests: Default::default(),
            completed_requests: Default::default(),
        }
    }
}

impl CacheExchange {
    async fn update_persisted_cache(&self, request_hash: u64, response: BaseResponse) {
        let mut completed_requests = self.completed_requests.lock().await;

        completed_requests.insert(request_hash, response);

        let document = window().and_then(|m| m.document()).unwrap_throw();

        if let Some(m) = document.head() {
            let m: &Element = m.as_ref();

            if let Ok(nodes) = m.query_selector_all("#yew-query-request-cache") {
                for i in 0..nodes.length() {
                    if let Some(node) = nodes.get(i) {
                        if let Some(m) = node.parent_node() {
                            m.remove_child(&node).unwrap();
                        }
                    }
                }
            }

            let mut serde_requests = HashMap::new();

            for (k, v) in completed_requests.iter() {
                serde_requests.insert(*k, SerdeResponse::from(v.clone()));
            }

            let next_cache = document
                .create_element("script")
                .unwrap_throw()
                .dyn_into::<HtmlScriptElement>()
                .unwrap_throw();
            next_cache.set_type("application/json");

            next_cache.set_id("yew-query-request-cache");

            next_cache
                .set_text(&*serde_json::to_string(&serde_requests).unwrap_throw())
                .unwrap_throw();

            m.append_child(next_cache.as_ref()).unwrap();
        }
    }
}

#[async_trait(?Send)]
impl Exchange for CacheExchange {
    async fn fetch(
        &self,
        _client: Rc<Client>,
        request: Request,
        forward: Box<
            dyn (FnOnce(Request) -> LocalBoxFuture<'static, InternalResult<BaseResponse>>)
                + 'static,
        >,
    ) -> InternalResult<BaseResponse> {
        let mut h = AHasher::new_with_keys(1234, 5678);
        request.hash(&mut h);
        let key = h.finish();

        let fur = {
            let requests = self.requests.clone();
            let mut requests = requests.lock().await;

            if let Some(m) = requests.get(&key).cloned() {
                m
            } else {
                let m = forward(request.clone()).shared();

                // Only cache Get requests.
                if request.method() == Method::Get {
                    requests.insert(key, m.clone());
                }

                m
            }
        };

        let resp = fur.await?;

        // Only cache Get requests.
        if request.method() == Method::Get {
            self.update_persisted_cache(key, resp.clone()).await;
        }

        Ok(resp)
    }
}
