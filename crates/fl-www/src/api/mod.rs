use messages::ResponseError;
use once_cell::sync::Lazy;
use reqwest::{Client, Url};
use thiserror::Error;

use crate::prelude::*;

mod replies;
mod residents;

static BASE_URL: Lazy<Url> = Lazy::new(|| {
    option_env!("FL_WWW_BACKEND_PREFIX")
        .unwrap_throw()
        .parse()
        .unwrap_throw()
});

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub use replies::{RepliesQuery, RepliesQueryInput};
pub use residents::{CurrentResidentQuery, ExchangeTokenMutation};

#[derive(Debug, Error, PartialEq, Clone)]
pub enum QueryError {
    #[error("failed to communicate with server")]
    Server(ResponseError),

    #[error("unknown server error")]
    ServerOther,

    #[error("forbidden")]
    Forbidden,
}
