use serde::{Deserialize, Serialize};
use stellation_bridge::routines::{BridgedMutation, BridgedQuery};

use crate::{messages, RoutineError};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResidentQueryInput {
    Id(u64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentQuery {
    pub content: messages::Resident,
}

impl BridgedQuery for ResidentQuery {
    type Error = RoutineError;
    type Input = ResidentQueryInput;

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentResidentQuery {
    pub content: Option<messages::Resident>,
}

impl BridgedQuery for CurrentResidentQuery {
    type Error = RoutineError;
    type Input = ();

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExchangeTokenMutation {
    pub content: messages::AccessToken,
}

impl BridgedMutation for ExchangeTokenMutation {
    type Error = RoutineError;
    type Input = messages::AccessTokenInput;

    fn into_mutation_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}
