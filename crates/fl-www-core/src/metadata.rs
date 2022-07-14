use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WritingMetadata {
    pub slug: String,
    pub lang: Language,
    pub date: NaiveDate,
    pub title: String,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub summary: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Metadata {
    #[builder(setter(into))]
    writings: Vec<WritingMetadata>,
}

impl Metadata {
    pub fn writings(&self) -> &[WritingMetadata] {
        &self.writings
    }
}

impl FromStr for Metadata {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}
