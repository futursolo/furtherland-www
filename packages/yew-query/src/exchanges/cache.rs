use std::collections::HashMap;
use std::rc::Rc;

use futures::future::FutureExt;
use futures::future::Shared;
use futures::lock::Mutex;

use crate::client::Client;
use crate::error::InternalResult;
use crate::request::{Method, Request};

use super::*;

type RequestMap = HashMap<Request, Shared<LocalBoxFuture<'static, InternalResult<BaseResponse>>>>;

#[derive(Debug, Default)]
pub struct CacheExchange {
    requests: Rc<Mutex<RequestMap>>,
    completed_requests: Rc<Mutex<HashMap<Request, BaseResponse>>>,
}

impl CacheExchange {
    async fn update_persisted_cache(&self, _request: Request, _response: BaseResponse) {}
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
        let fur = {
            let requests = self.requests.clone();
            let mut requests = requests.lock().await;

            if let Some(m) = requests.get(&request).cloned() {
                m
            } else {
                let m = forward(request.clone()).shared();

                // Only cache Get requests.
                if request.method() == Method::Get {
                    requests.insert(request.clone(), m.clone());
                }

                m
            }
        };

        let resp = fur.await?;

        self.update_persisted_cache(request, resp.clone()).await;

        Ok(resp)
    }
}
