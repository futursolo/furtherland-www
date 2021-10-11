use std::convert::Infallible;
use std::rc::Rc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<E>
where
    E: std::error::Error + 'static,
{
    #[error("Failed to parse")]
    Parse(#[source] E),

    #[error("Response didn't succeed")]
    Response(web_sys::Response),

    #[error("Failed to communicate with server")]
    Fetch(wasm_bindgen::JsValue),

    #[error("Web API failed")]
    Web(Option<wasm_bindgen::JsValue>),
}

impl Error<Infallible> {
    pub fn cast_parse_err<E: std::error::Error + 'static>(self) -> Error<E> {
        match self {
            Self::Parse(_) => panic!(),
            Self::Response(m) => Error::Response(m),
            Self::Fetch(m) => Error::Fetch(m),
            Self::Web(m) => Error::Web(m),
        }
    }
}

pub type Result<T, E> = std::result::Result<T, Rc<Error<E>>>;
