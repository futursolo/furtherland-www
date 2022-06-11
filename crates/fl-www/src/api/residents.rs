use std::rc::Rc;

use async_trait::async_trait;
use atoms::TokenState;
use bounce::query::{Mutation, MutationResult, Query, QueryResult};
use bounce::BounceStates;
use futures::future::TryFutureExt;
use messages::{Resident, Response};

use super::{QueryError, BASE_URL};
use crate::prelude::*;

// #[async_trait(?Send)]
// pub trait Query: PartialEq {
//     type Input: Hash + Eq + 'static;
//     type Error: 'static + std::error::Error + PartialEq + Clone;

//     async fn query(states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self>;
// }

#[derive(Debug, PartialEq)]
pub struct CurrentResidentQuery {
    pub content: Option<Resident>,
}

#[async_trait(?Send)]
impl Query for CurrentResidentQuery {
    type Error = QueryError;
    type Input = ();

    async fn query(states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        let client = reqwest::Client::new();

        let token = match states.get_atom_value::<TokenState>().inner.as_ref() {
            Some(m) => m.clone(),
            None => return Ok(CurrentResidentQuery { content: None }.into()),
        };

        let resp = client
            .get(BASE_URL.join("/residents/myself").unwrap_throw())
            .bearer_auth(token)
            .send()
            .and_then(|m| m.json::<Response<Resident>>())
            .map_err(|_e| QueryError::ServerOther)
            .await?;

        match resp {
            Response::Success { content } => Ok(CurrentResidentQuery {
                content: Some(content),
            }
            .into()),
            Response::Failed { error } => Err(QueryError::Server(error)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExchangeTokenMutation {
    pub content: messages::AccessToken,
}

#[async_trait(?Send)]
impl Mutation for ExchangeTokenMutation {
    type Error = QueryError;
    type Input = messages::AccessTokenInput;

    async fn run(_states: &BounceStates, input: Rc<Self::Input>) -> MutationResult<Self> {
        let client = reqwest::Client::new();

        let resp = client
            .post(
                BASE_URL
                    .join("/residents/_oauth_access_token")
                    .unwrap_throw(),
            )
            .json(&input)
            .send()
            .and_then(|m| m.json::<Response<messages::AccessToken>>())
            .await
            .map_err(|_e| QueryError::ServerOther)?;

        match resp {
            Response::Success { content } => Ok(Self { content }.into()),
            Response::Failed { error } => Err(QueryError::Server(error)),
        }
    }
}
