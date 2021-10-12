use std::convert::Infallible;

use typed_builder::TypedBuilder;
use web_sys::window;

use crate::client::Client;
use crate::error::Error;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Hash)]
pub struct Request {
    #[builder(setter(into))]
    url: String,
}

impl Request {
    pub(crate) fn to_fetch_request(
        &self,
        client: &Client,
    ) -> std::result::Result<web_sys::Request, Error<Infallible>> {
        let url = if let Some(m) = client.base_url() {
            web_sys::Url::new_with_base(&self.url, m)
        } else {
            let window = window().ok_or(Error::Web(None))?;
            let base_url = window.location().href().map_err(|e| Error::Web(Some(e)))?;

            web_sys::Url::new_with_base(&self.url, &base_url)
        }
        .map(|m| m.href())
        .map_err(|e| Error::Web(Some(e)))?;

        web_sys::Request::new_with_str(&url).map_err(|e| Error::Web(Some(e)))
    }
}
