use std::rc::Rc;

use futures::future::LocalBoxFuture;
use typed_builder::TypedBuilder;
use wasm_bindgen::throw_str;

use crate::error::InternalResult;
use crate::exchanges::{CacheExchange, Exchange, FetchExchange};
use crate::request::Request;
use crate::response::BaseResponse;
use crate::utils::Id;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Client {
    #[builder(setter(skip), default)]
    id: Id,

    #[builder(setter(into, strip_option), default)]
    base_url: Option<String>,

    #[builder(setter(into), default = Client::default_exchanges())]
    exchanges: Vec<Rc<dyn Exchange>>,
}

impl PartialEq for Client {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Client {
    pub fn base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub(crate) fn default_exchanges() -> Vec<Rc<dyn Exchange>> {
        vec![
            Rc::new(CacheExchange::default()),
            Rc::new(FetchExchange::default()),
        ]
    }

    pub(crate) async fn fetch(self: Rc<Self>, request: Request) -> InternalResult<BaseResponse> {
        let mut fur = Box::new(
            |_req: Request| -> LocalBoxFuture<'static, InternalResult<BaseResponse>> {
                Box::pin(async {
                    throw_str("unreachable!");
                })
            },
        )
            as Box<
                dyn (FnOnce(Request) -> LocalBoxFuture<'static, InternalResult<BaseResponse>>)
                    + 'static,
            >;

        for exchange in self.exchanges.iter().rev() {
            let self_ = self.clone();
            let exchange = exchange.clone();
            let next_fur = Box::new(
                move |request: Request| -> LocalBoxFuture<'static, InternalResult<BaseResponse>> {
                    Box::pin(async move { exchange.fetch(self_, request, fur).await })
                },
            )
                as Box<
                    dyn (FnOnce(Request) -> LocalBoxFuture<'static, InternalResult<BaseResponse>>)
                        + 'static,
                >;

            fur = next_fur;
        }

        fur(request).await
    }
}
