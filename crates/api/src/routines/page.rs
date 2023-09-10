use fl_www_core::prelude::*;
use serde::{Deserialize, Serialize};
use stellation_bridge::routines::BridgedQuery;
use typed_builder::TypedBuilder;

use crate::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, TypedBuilder)]
pub struct PageQueryInput {
    pub slug: String,
    pub lang: Language,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageQuery {
    pub content: String,
}

impl BridgedQuery for PageQuery {
    type Error = Error;
    type Input = PageQueryInput;

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}
