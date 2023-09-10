use chrono::NaiveDate;
use fl_www_core::prelude::*;
use serde::{Deserialize, Serialize};
use stellation_bridge::routines::BridgedQuery;
use typed_builder::TypedBuilder;

use crate::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, TypedBuilder)]
pub struct WritingQueryInput {
    pub slug: String,
    pub lang: Language,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WritingQuery {
    pub date: NaiveDate,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
}

impl BridgedQuery for WritingQuery {
    type Error = Error;
    type Input = WritingQueryInput;

    fn into_query_error(_e: stellation_bridge::BridgeError) -> Self::Error {
        Error::Network
    }
}
