use serde::{Deserialize, Serialize};
use stellation_bridge::routines::BridgedQuery;

use crate::markdown::types::Document;
use crate::RoutineError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct MarkdownQueryInput {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarkdownQuery {
    pub value: Document,
}

impl BridgedQuery for MarkdownQuery {
    type Error = RoutineError;
    type Input = MarkdownQueryInput;

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}
