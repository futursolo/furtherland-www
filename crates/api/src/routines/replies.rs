use fl_www_core::messages::{Replies, Reply};
use fl_www_core::prelude::*;
use serde::{Deserialize, Serialize};
use stellation_bridge::routines::{BridgedMutation, BridgedQuery};

use crate::Error;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct RepliesQueryInput {
    pub lang: Language,
    pub slug: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepliesQuery {
    pub content: Replies,
}

impl BridgedQuery for RepliesQuery {
    type Error = Error;
    type Input = RepliesQueryInput;

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateReplyInput {
    pub content: String,
    pub slug: String,
    pub lang: Language,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateReplyMutation {
    pub content: Reply,
}

impl BridgedMutation for CreateReplyMutation {
    type Error = Error;
    type Input = CreateReplyInput;

    fn into_mutation_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}
