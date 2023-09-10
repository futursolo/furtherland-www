use std::ops::Deref;

use fl_www_core::prelude::*;
use serde::{Deserialize, Serialize};
use stellation_bridge::routines::BridgedQuery;

use crate::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetadataQuery {
    pub value: (),
}

impl BridgedQuery for MetadataQuery {
    type Error = Error;
    type Input = ();

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}

impl Deref for MetadataQuery {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
