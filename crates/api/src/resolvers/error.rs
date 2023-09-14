use thiserror::Error;

use crate::RoutineError;

#[derive(Debug, Error, Clone)]
pub(crate) enum ResolverError {
    #[error("error from GitHub API")]
    GitHub,

    #[error("content not found")]
    NotFound,

    #[error("data integrity error, due to {}", .0)]
    DataIntegrity(String),

    #[error("database error")]
    Database(#[from] sea_orm::DbErr),

    // #[error("request too large")]
    // RequestTooLarge,

    // #[error("method not allowed")]
    // MethodNotAllowed,
    #[error("forbidden: you do not have permission to perform the action")]
    Forbidden,
    // #[error("bad request: your request is not valid.")]
    // BadRequest,

    // #[error("bad request: the request content type is not supported or missing.")]
    // UnsupportedMedia,

    // #[error("unknown error")]
    // Other,
}

impl From<ResolverError> for RoutineError {
    fn from(value: ResolverError) -> Self {
        match value {
            ResolverError::GitHub => Self::GitHub,
            ResolverError::DataIntegrity(_) => Self::ServerOther,
            ResolverError::Database(_) => Self::ServerOther,
            ResolverError::Forbidden => Self::Forbidden,
            ResolverError::NotFound => Self::NotFound,
        }
    }
}

pub(crate) type ResolverResult<T> = std::result::Result<T, ResolverError>;
