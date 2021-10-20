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

pub type Result<T, E> = std::result::Result<T, Rc<Error<E>>>;
