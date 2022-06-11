use js_sys::JSON;
use std::rc::Rc;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use web_sys::Headers;

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct BaseResponse {
    pub(crate) data: Rc<Vec<u8>>,
    pub(crate) headers: Headers,
}

impl BaseResponse {
    pub(crate) fn into_response<T, E>(self) -> Result<Response<T>, E>
    where
        T: FromStr<Err = E> + Clone + 'static,
        E: std::error::Error + 'static,
    {
        let data = String::from_utf8(self.data.to_vec())
            .map_err(|e| Rc::new(Error::from(e)))?
            .parse::<T>()
            .map_err(|e| Rc::new(Error::Parse(e)))?;
        Ok(Response {
            data: Rc::new(data),
            headers: self.headers,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeResponse {
    pub(crate) data: Rc<Vec<u8>>,
    pub(crate) headers: Vec<(String, String)>,
}

impl From<BaseResponse> for SerdeResponse {
    fn from(m: BaseResponse) -> Self {
        let mut headers = Vec::new();

        for item in js_sys::try_iter(&m.headers)
            .ok()
            .flatten()
            .expect_throw("not an iter?")
        {
            let item = item.expect_throw("What?");

            let serialized_headers: String = JSON::stringify(&item)
                .expect_throw("serialized headers")
                .into();

            let [name, value]: [String; 2] = serde_json::from_str(&serialized_headers)
                .expect_throw("deserializable serialized headers");

            headers.push((name, value));
        }

        Self {
            data: m.data,
            headers,
        }
    }
}

impl From<SerdeResponse> for BaseResponse {
    fn from(m: SerdeResponse) -> Self {
        let headers = Headers::new().expect_throw("failed to create headers");

        for (name, value) in m.headers.iter() {
            headers
                .append(name, value)
                .expect_throw("failed to append headers");
        }

        Self {
            data: m.data,
            headers,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response<T>
where
    T: Clone + 'static,
{
    // inner: Rc<reqwest::Response>,
    pub(crate) data: Rc<T>,
    pub(crate) headers: Headers,
}

impl<T> Response<T>
where
    T: Clone + 'static,
{
    pub fn data(&self) -> Rc<T> {
        self.data.clone()
    }

    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }
}
