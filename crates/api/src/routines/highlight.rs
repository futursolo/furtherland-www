use serde::{Deserialize, Serialize};
use stellation_bridge::routines::BridgedQuery;

use crate::core::styling::{Colour, ThemeKind};
use crate::RoutineError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HighlightInput {
    pub content: String,
    pub language: String,
    pub theme_kind: ThemeKind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct HighlightOutput {
    pub fragments: Vec<(Colour, String)>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HighlightQuery {
    pub value: Option<HighlightOutput>,
}

impl BridgedQuery for HighlightQuery {
    type Error = RoutineError;
    type Input = HighlightInput;

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}
