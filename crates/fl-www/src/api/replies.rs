use std::rc::Rc;

use async_trait::async_trait;
use atoms::TokenState;
use bounce::query::{Mutation, MutationResult, Query, QueryResult};
use bounce::BounceStates;
use futures::TryFutureExt;
use messages::{Replies, Reply, ReplyInput, Response};

use super::{QueryError, BASE_URL, CLIENT};
use crate::prelude::*;

// #[async_trait(?Send)]
// pub trait Query: PartialEq {
//     type Input: Hash + Eq + 'static;
//     type Error: 'static + std::error::Error + PartialEq + Clone;

//     async fn query(states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self>;
// }

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
    type Error = QueryError;
    type Input = RepliesQueryInput;

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
        .map_err(|_e| QueryError::ServerOther)
        .await?;

        match resp {
            Response::Success { content } => Ok(RepliesQuery { content }.into()),
            Response::Failed { error } => Err(QueryError::Server(error)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateReplyMutation {
    pub content: Reply,
}

#[async_trait(?Send)]
impl Mutation for CreateReplyMutation {
    type Error = QueryError;
    type Input = ReplyInput;

    async fn run(states: &BounceStates, input: Rc<Self::Input>) -> MutationResult<Self> {
        let token = match states.get_atom_value::<TokenState>().inner.as_ref() {
            Some(m) => m.clone(),
            None => return Err(QueryError::Forbidden),
        };

        let resp = CLIENT
            .post(BASE_URL.join("/replies").unwrap_throw())
            .bearer_auth(token)
            .json(&input)
            .send()
            .and_then(|m| m.json::<Response<messages::Reply>>())
            .await
            .map_err(|_e| QueryError::ServerOther)?;

        match resp {
            Response::Success { content } => Ok(Self { content }.into()),
            Response::Failed { error } => Err(QueryError::Server(error)),
        }
    }
}
