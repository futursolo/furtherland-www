use std::convert::Infallible;
use std::rc::Rc;

use thiserror::Error;
use wasm_bindgen::throw_str;

#[derive(Error, Debug, Clone)]
pub enum InternalError {
    #[error("Response didn't succeed")]
    Response(web_sys::Response),

    #[error("Failed to communicate with server")]
    Fetch(wasm_bindgen::JsValue),

    #[error("Web API failed")]
    Web(Option<wasm_bindgen::JsValue>),
}

pub(crate) type InternalResult<T> = std::result::Result<T, InternalError>;

#[derive(Error, Debug)]
pub enum Error<E>
where
    E: std::error::Error + 'static,
{
    #[error("Failed to parse")]
    Parse(#[source] E),

    #[error("Failed to parse response as String")]
    Unicode(#[from] std::string::FromUtf8Error),

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
            Self::Parse(_) => throw_str("Unreachable!"),
            Self::Response(m) => Error::Response(m),
            Self::Unicode(m) => Error::Unicode(m),
            Self::Fetch(m) => Error::Fetch(m),
            Self::Web(m) => Error::Web(m),
        }
    }
}

impl From<InternalError> for Error<Infallible> {
    fn from(int_err: InternalError) -> Self {
        match int_err {
            InternalError::Response(m) => Self::Response(m),
            InternalError::Fetch(m) => Self::Fetch(m),
            InternalError::Web(m) => Self::Web(m),
        }
    }
}

pub type Result<T, E> = std::result::Result<T, Rc<Error<E>>>;
