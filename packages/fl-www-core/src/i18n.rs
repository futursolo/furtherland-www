use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Chinese => "zh",
            Self::English => "en",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().trim() {
            "zh" => Ok(Self::Chinese),
            "en" => Ok(Self::English),
            _ => Err(Error::ParseStr {
                target_kind: "Language".to_string(),
                reason: "Value is not valid.".to_string(),
            }),
        }
    }
}
