use futures::future::{ready, TryFutureExt};
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use crate::client::Client;
use crate::error::InternalError;
use crate::request::Request;
use crate::response::BaseResponse;

use super::*;

#[derive(Debug, Default)]
pub struct FetchExchange;

#[async_trait(?Send)]
impl Exchange for FetchExchange {
    async fn fetch(
        &self,
        client: Rc<Client>,
        request: Request,
        _forward: Box<
            dyn (FnOnce(Request) -> LocalBoxFuture<'static, InternalResult<BaseResponse>>)
                + 'static,
        >,
    ) -> InternalResult<BaseResponse> {
        let window = window().ok_or(InternalError::Web(None))?;
        let req = request.to_fetch_request(&client)?;
        let resp = JsFuture::from(window.fetch_with_request(&req))
            .await
            .and_then(|m| m.dyn_into::<web_sys::Response>())
            .map_err(|e| InternalError::Web(Some(e)))?;

        if resp.status() >= 400 {
            return Err(InternalError::Response(resp));
        }

        let headers = resp.headers().to_owned();
        let data = ready(resp.blob().map(JsFuture::from))
            .try_flatten()
            .await
            .map_err(InternalError::Fetch)?
            .dyn_into::<web_sys::Blob>()
            .map(|m| JsFuture::from(m.array_buffer()))
            .map_err(|e| InternalError::Web(Some(e)))?
            .await
            .map(|m| Uint8Array::new(&m).to_vec())
            .map_err(|e| InternalError::Web(Some(e)))?;

        Ok(BaseResponse {
            data: data.into(),
            headers,
        })
    }
}
