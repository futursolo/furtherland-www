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
                    requests.insert(request, m.clone());
                }

                m
            }
        };

        fur.await
    }
}
