use std::convert::Infallible;

use thiserror::Error;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::{self, Reply, Response};
use warp::Rejection;

use crate::encoding::Encoding;
use crate::prelude::*;

#[derive(Debug, Error, Clone)]
pub(crate) enum HttpError {
    #[error("error from GitHub API.")]
    GitHub,

    #[error("page not found.")]
    NotFound,

    #[error("data integrity error, due to {}.", .0)]
    DataIntegrity(String),

    #[error("database error.")]
    Database(#[from] sea_orm::DbErr),

    #[error("request too large")]
    RequestTooLarge,

    #[error("method not allowed")]
    MethodNotAllowed,

    #[error("forbidden: you do not have permission to perform the action.")]
    Forbidden,

    #[error("bad request: your request is not valid.")]
    BadRequest,

    #[error("bad request: the request content type is not supported or missing.")]
    UnsupportedMedia,

    #[error("unknown error")]
    Other,
}

impl Reject for HttpError {}

impl HttpError {
    pub fn to_reply(&self, encoding: Encoding) -> impl Reply + Send + 'static {
        log::warn!("error occurred: {:?}", self);

        use messages::{Response, ResponseError};

        match self {
            Self::Other => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 5004 },
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),

            Self::GitHub => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 5002 },
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),

            Self::DataIntegrity(_) => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 5006 },
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),

            Self::Database(_) => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 5005 },
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),

            Self::RequestTooLarge => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 413 },
                }),
                StatusCode::PAYLOAD_TOO_LARGE,
            ),

            Self::MethodNotAllowed => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 405 },
                }),
                StatusCode::METHOD_NOT_ALLOWED,
            ),

            Self::NotFound => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 404 },
                }),
                StatusCode::NOT_FOUND,
            ),

            Self::Forbidden => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 403 },
                }),
                StatusCode::FORBIDDEN,
            ),

            Self::BadRequest => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 400 },
                }),
                StatusCode::BAD_REQUEST,
            ),

            Self::UnsupportedMedia => reply::with_status(
                encoding.reply(&Response::<()>::Failed {
                    error: ResponseError { code: 415 },
                }),
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ),
        }
    }

    pub async fn recover_to_error(
        err: Rejection,
    ) -> std::result::Result<HttpResult<Response>, Infallible> {
        log::warn!("request rejected: {:?}", err);

        if let Some(m) = err.find::<Self>() {
            return Ok(Err(m.clone()));
        }

        if err
            .find::<warp::filters::body::BodyDeserializeError>()
            .is_some()
            || err.find::<warp::reject::MissingHeader>().is_some()
            || err.find::<warp::reject::InvalidHeader>().is_some()
            || err.find::<warp::reject::UnsupportedMediaType>().is_some()
            || err.find::<warp::reject::MissingCookie>().is_some()
            || err.find::<warp::reject::InvalidQuery>().is_some()
            || err.find::<warp::reject::LengthRequired>().is_some()
        {
            return Ok(Err(Self::BadRequest));
        }

        if err.find::<warp::reject::MethodNotAllowed>().is_some() {
            return Ok(Err(Self::MethodNotAllowed));
        }

        if err.find::<warp::reject::PayloadTooLarge>().is_some() {
            return Ok(Err(Self::RequestTooLarge));
        }

        if err.is_not_found() {
            return Ok(Err(Self::NotFound));
        }

        Ok(Err(Self::Other))
    }
}

pub(crate) type HttpResult<T> = std::result::Result<T, HttpError>;
