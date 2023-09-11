use std::ops::Deref;

use serde::{Deserialize, Serialize};
use stellation_bridge::routines::BridgedQuery;

use crate::core::prelude::*;
use crate::RoutineError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetadataQuery {
    pub value: Metadata,
}

impl BridgedQuery for MetadataQuery {
    type Error = RoutineError;
    type Input = ();

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        RoutineError::Network
    }
}

impl Deref for MetadataQuery {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
