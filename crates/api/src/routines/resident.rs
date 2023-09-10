use serde::{Deserialize, Serialize};
use stellation_bridge::routines::{BridgedMutation, BridgedQuery};

use crate::{messages, Error};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentResidentQuery {
    pub inner: Option<messages::Resident>,
}

impl BridgedQuery for CurrentResidentQuery {
    type Error = Error;
    type Input = ();

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExchangeTokenMutation {
    pub content: messages::AccessToken,
}

impl BridgedMutation for ExchangeTokenMutation {
    type Error = Error;
    type Input = messages::AccessTokenInput;

    fn into_mutation_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}
