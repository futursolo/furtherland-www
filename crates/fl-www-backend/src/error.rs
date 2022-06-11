use std::convert::Infallible;

use thiserror::Error;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::{self, Reply};
use warp::Rejection;

use crate::prelude::*;

#[derive(Debug, Error)]
pub(crate) enum HttpError {
    #[error("error from GitHub API.")]
    GitHub,

    #[error("page not found.")]
    NotFound,

    #[error("forbidden: you do not have permission to perform the action.")]
    Forbidden,

    #[error("bad request: your request is not valid.")]
    BadRequest,

    #[error("unknown error")]
    Other,
}

impl Reject for HttpError {}

impl HttpError {
    pub fn to_reply(&self) -> impl Reply + Send + 'static {
        use messages::{Response, ResponseError};

        match self {
            Self::Other => reply::with_status(
                reply::json(&Response::<()>::Failed {
                    error: ResponseError { code: 5004 },
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),

            Self::GitHub => reply::with_status(
                reply::json(&Response::<()>::Failed {
                    error: ResponseError { code: 5002 },
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),

            Self::NotFound => reply::with_status(
                reply::json(&Response::<()>::Failed {
                    error: ResponseError { code: 404 },
                }),
                StatusCode::NOT_FOUND,
            ),

            Self::Forbidden => reply::with_status(
                reply::json(&Response::<()>::Failed {
                    error: ResponseError { code: 403 },
                }),
                StatusCode::FORBIDDEN,
            ),

            Self::BadRequest => reply::with_status(
                reply::json(&Response::<()>::Failed {
                    error: ResponseError { code: 400 },
                }),
                StatusCode::BAD_REQUEST,
            ),
        }
    }

    pub async fn handle_rejection(
        err: Rejection,
    ) -> std::result::Result<impl Reply + Send + 'static, Infallible> {
        if let Some(m) = err.find::<Self>() {
            return Ok(m.to_reply());
        }

        if err.is_not_found() {
            return Ok(Self::NotFound.to_reply());
        }

        Ok(Self::Other.to_reply())
    }
}

pub(crate) type HttpResult<T> = std::result::Result<T, HttpError>;
