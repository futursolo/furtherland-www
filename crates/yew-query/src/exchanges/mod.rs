use std::fmt;
use std::rc::Rc;

use async_trait::async_trait;
use futures::future::LocalBoxFuture;

use crate::client::Client;
use crate::error::InternalResult;
use crate::request::Request;
use crate::response::BaseResponse;

mod cache;
mod fetch;

pub use cache::CacheExchange;
pub use fetch::FetchExchange;

#[async_trait(?Send)]
pub trait Exchange: fmt::Debug {
    async fn fetch(
        &self,
        client: Rc<Client>,
        request: Request,
        forward: Box<
            dyn (FnOnce(Request) -> LocalBoxFuture<'static, InternalResult<BaseResponse>>)
                + 'static,
        >,
    ) -> InternalResult<BaseResponse>;
}
