use std::rc::Rc;

use crate::prelude::*;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce_query::{Query, QueryResult};
use futures::TryFutureExt;
use thiserror::Error;

use messages::{Replies, Response, ResponseError};

use super::BASE_URL;

// #[async_trait(?Send)]
// pub trait Query: PartialEq {
//     type Input: Hash + Eq + 'static;
//     type Error: 'static + std::error::Error + PartialEq + Clone;

//     async fn query(states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self>;
// }

#[derive(Debug, Error, PartialEq, Clone)]
pub enum RepliesQueryError {
    #[error("failed to communicate with server")]
    Server(ResponseError),

    #[error("unknown server error")]
    ServerOther,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RepliesQueryInput {
    pub lang: Language,
    pub slug: String,
}

#[derive(Debug, PartialEq)]
pub struct RepliesQuery {
    pub content: Replies,
}

#[async_trait(?Send)]
impl Query for RepliesQuery {
    type Input = RepliesQueryInput;
    type Error = RepliesQueryError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        let resp = reqwest::get(
            BASE_URL
                .join(&format!(
                    "/replies/{lang}/{slug}/",
                    lang = input.lang,
                    slug = input.slug
                ))
                .unwrap_throw(),
        )
        .and_then(|m| m.json::<Response<Replies>>())
        .map_err(|_e| RepliesQueryError::ServerOther)
        .await?;

        match resp {
            Response::Success { content } => Ok(RepliesQuery { content }.into()),
            Response::Failed { error } => Err(RepliesQueryError::Server(error)),
        }
    }
}
