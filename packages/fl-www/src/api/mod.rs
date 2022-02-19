use crate::prelude::*;

use once_cell::sync::Lazy;
use reqwest::Url;
use thiserror::Error;

use messages::ResponseError;

mod replies;
mod residents;

static BASE_URL: Lazy<Url> = Lazy::new(|| {
    option_env!("FL_WWW_BACKEND_PREFIX")
        .unwrap_throw()
        .parse()
        .unwrap_throw()
});

pub use replies::{RepliesQuery, RepliesQueryInput};
pub use residents::CurrentResidentQuery;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum QueryError {
    #[error("failed to communicate with server")]
    Server(ResponseError),

    #[error("unknown server error")]
    ServerOther,
}
