use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use web_sys::window;

use crate::client::Client;
use crate::error::{InternalError, InternalResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Method {
    Get,
    Head,
}

impl Default for Method {
    fn default() -> Self {
        Method::Get
    }
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Request {
    #[builder(setter(into))]
    url: String,

    #[builder(default)]
    method: Method,
}

impl Request {
    pub fn method(&self) -> Method {
        self.method
    }
}

impl Request {
    pub(crate) fn to_fetch_request(&self, client: &Client) -> InternalResult<web_sys::Request> {
        let url = if let Some(m) = client.base_url() {
            web_sys::Url::new_with_base(&self.url, m)
        } else {
            let window = window().ok_or(InternalError::Web(None))?;
            let base_url = window
                .location()
                .href()
                .map_err(|e| InternalError::Web(Some(e)))?;

            web_sys::Url::new_with_base(&self.url, &base_url)
        }
        .map(|m| m.href())
        .map_err(|e| InternalError::Web(Some(e)))?;

        web_sys::Request::new_with_str(&url).map_err(|e| InternalError::Web(Some(e)))
    }
}
