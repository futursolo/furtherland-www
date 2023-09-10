use serde::{Deserialize, Serialize};
use stellation_bridge::routines::BridgedQuery;

// use crate::core::markdown::Root;
use crate::RoutineError;
pub mod types;

pub use types::Root;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct MarkdownQueryInput {
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarkdownQuery {
    pub value: Root,
}

impl BridgedQuery for MarkdownQuery {
    type Error = RoutineError;
    type Input = MarkdownQueryInput;

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}
