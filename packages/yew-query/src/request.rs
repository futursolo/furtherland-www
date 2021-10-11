use std::convert::Infallible;

use typed_builder::TypedBuilder;

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
            format!("{}{}", m, self.url)
        } else {
            self.url.to_string()
        };

        web_sys::Request::new_with_str(&url).map_err(|e| Error::Web(Some(e)))
    }
}
