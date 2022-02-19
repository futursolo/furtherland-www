use std::rc::Rc;

use crate::prelude::*;
use atoms::TokenState;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use messages::{Resident, Response};

use super::{QueryError, BASE_URL};

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
    type Input = ();
    type Error = QueryError;

    async fn query(states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        let token = match states.get_atom_value::<TokenState>().inner.as_ref() {
            Some(m) => m,
            None => return Ok(CurrentResidentQuery { content: None }.into()),
        };

        todo!()

        // let resp = reqwest::get(
        //     BASE_URL
        //         .join(&format!(
        //             "/replies/{lang}/{slug}/",
        //             lang = input.lang,
        //             slug = input.slug
        //         ))
        //         .unwrap_throw(),
        // )
        // .and_then(|m| m.json::<Response<Replies>>())
        // .map_err(|_e| ResidentQueryError::ServerOther)
        // .await?;

        // match resp {
        //     Response::Success { content } => Ok(RepliesQuery { content }.into()),
        //     Response::Failed { error } => Err(ResidentQueryError::Server(error)),
        // }
    }
}
