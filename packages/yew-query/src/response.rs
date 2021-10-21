use std::rc::Rc;
use std::str::FromStr;

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
