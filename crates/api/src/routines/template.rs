use serde::{Deserialize, Serialize};
use stellation_bridge::routines::{BridgedMutation, BridgedQuery};
use time::OffsetDateTime;

use super::RoutineError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerTimeQuery {
    pub value: OffsetDateTime,
}

impl BridgedQuery for ServerTimeQuery {
    type Error = RoutineError;
    type Input = ();

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GreetingMutation {
    pub message: String,
}

impl BridgedMutation for GreetingMutation {
    type Error = RoutineError;
    type Input = String;

    fn into_mutation_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}
