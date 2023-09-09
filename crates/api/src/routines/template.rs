use serde::{Deserialize, Serialize};
use stellation_bridge::routines::{BridgedMutation, BridgedQuery};
use time::OffsetDateTime;

use super::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerTimeQuery {
    pub value: OffsetDateTime,
}

impl BridgedQuery for ServerTimeQuery {
    type Error = Error;
    type Input = ();

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GreetingMutation {
    pub message: String,
}

impl BridgedMutation for GreetingMutation {
    type Error = Error;
    type Input = String;

    fn into_mutation_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}
