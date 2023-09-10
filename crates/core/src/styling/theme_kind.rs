use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub enum ThemeKind {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
}

impl fmt::Display for ThemeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ThemeKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    // pub fn current() -> Self {
    //     if let Some(m) = window()
    //         .match_media("(prefers-color-scheme: dark)")
    //         .ok()
    //         .and_then(|m| m)
    //     {
    //         if m.matches() {
    //             ThemeKind::Dark
    //         } else {
    //             ThemeKind::Light
    //         }
    //     } else {
    //         ThemeKind::Light
    //     }
    // }

    pub fn alternative(&self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

impl FromStr for ThemeKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err(Error::ParseStr {
                target_kind: "ThemeKind".to_string(),
                reason: "Value is not valid.".to_string(),
            }),
        }
    }
}
