use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RoutineError {
    #[error("error from GitHub API")]
    GitHub,

    #[error("content not found")]
    NotFound,

    #[error("unknown server error")]
    ServerOther,

    #[error("forbidden")]
    Forbidden,

    #[error("failed to communicate with server.")]
    Network,
}
