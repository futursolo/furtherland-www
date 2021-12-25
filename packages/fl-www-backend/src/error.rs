use thiserror::Error;
use worker::wasm_bindgen::UnwrapThrowExt;

use crate::prelude::*;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("error from Cloudflare Worker.")]
    Worker(#[from] worker::Error),
    #[error("error from GitHub API.")]
    GitHub,

    #[error("error from Cloudflare Worker KV.")]
    Kv(#[from] worker::kv::KvError),

    #[error("page not found.")]
    NotFound,

    #[error("forbidden: you do not have permission to perform the action.")]
    Forbidden,

    #[error("bad request: your request is not valid.")]
    BadRequest,
}

impl Error {
    pub fn into_response(self) -> worker::Response {
        match self {
            Error::Worker(_m) => worker::Response::from_json(&messages::Response::<()>::Failed {
                error: messages::ResponseError { code: 5001 },
            })
            .unwrap_throw()
            .with_status(500),
            Error::GitHub => worker::Response::from_json(&messages::Response::<()>::Failed {
                error: messages::ResponseError { code: 5002 },
            })
            .unwrap_throw()
            .with_status(500),
            Error::Kv(_m) => worker::Response::from_json(&messages::Response::<()>::Failed {
                error: messages::ResponseError { code: 5003 },
            })
            .unwrap_throw()
            .with_status(500),

            Error::NotFound => worker::Response::from_json(&messages::Response::<()>::Failed {
                error: messages::ResponseError { code: 404 },
            })
            .unwrap_throw()
            .with_status(404),

            Error::Forbidden => worker::Response::from_json(&messages::Response::<()>::Failed {
                error: messages::ResponseError { code: 403 },
            })
            .unwrap_throw()
            .with_status(403),

            Error::BadRequest => worker::Response::from_json(&messages::Response::<()>::Failed {
                error: messages::ResponseError { code: 400 },
            })
            .unwrap_throw()
            .with_status(400),
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
