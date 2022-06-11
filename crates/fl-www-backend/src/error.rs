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

    #[error("request too large")]
    RequestTooLarge,

    #[error("method not allowed")]
    MethodNotAllowed,

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

            Self::RequestTooLarge => reply::with_status(
                reply::json(&Response::<()>::Failed {
                    error: ResponseError { code: 413 },
                }),
                StatusCode::PAYLOAD_TOO_LARGE,
            ),

            Self::MethodNotAllowed => reply::with_status(
                reply::json(&Response::<()>::Failed {
                    error: ResponseError { code: 405 },
                }),
                StatusCode::METHOD_NOT_ALLOWED,
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

        if err
            .find::<warp::filters::body::BodyDeserializeError>()
            .is_some()
            || err.find::<warp::reject::MissingHeader>().is_some()
            || err.find::<warp::reject::InvalidHeader>().is_some()
            || err.find::<warp::reject::UnsupportedMediaType>().is_some()
            || err.find::<warp::reject::MissingCookie>().is_some()
            || err.find::<warp::reject::InvalidQuery>().is_some()
        {
            return Ok(Self::BadRequest.to_reply());
        }

        if err.find::<warp::reject::MethodNotAllowed>().is_some() {
            return Ok(Self::MethodNotAllowed.to_reply());
        }

        if err.find::<warp::reject::PayloadTooLarge>().is_some() {
            return Ok(Self::RequestTooLarge.to_reply());
        }

        if err.is_not_found() {
            return Ok(Self::NotFound.to_reply());
        }

        Ok(Self::Other.to_reply())
    }
}

pub(crate) type HttpResult<T> = std::result::Result<T, HttpError>;
