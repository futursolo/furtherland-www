use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[cfg_attr(
    feature = "backend",
    derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)
)]
#[cfg_attr(
    feature = "backend",
    sea_orm(rs_type = "String", db_type = "String(Some(12))")
)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize, Hash)]
pub enum Language {
    #[cfg_attr(feature = "backend", sea_orm(string_value = "zh"))]
    Chinese,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "en"))]
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
