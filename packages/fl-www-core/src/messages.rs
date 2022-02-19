use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::object_id::ObjectId;
use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResidencyStatus {
    // Disabled = -1,
    // Pending = 0,
    Resident = 10,
    Master = 100,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Resident {
    pub id: u64,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
}

impl Resident {
    pub fn status(&self) -> ResidencyStatus {
        if self.id == 11693215 {
            ResidencyStatus::Master
        } else {
            ResidencyStatus::Resident
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Replies {
    pub replies: Vec<Reply>,
    // None if is last item has been reached.
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Reply {
    pub id: ObjectId,
    pub slug: String,
    pub lang: Language,

    // The resident may be None if they have unregistered their GitHub account.
    pub resident: Option<Resident>,
    pub content: String,

    #[serde(default)]
    pub approved: Option<bool>,

    pub created_at: DateTime<Utc>,
}

impl Reply {
    /// Returns the key in the storage.
    pub fn key(&self) -> String {
        format!("{}:{}:{}", self.lang, self.slug, self.id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplyInput {
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatchReplyInput {
    #[serde(default)]
    pub approved: Option<bool>,
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessTokenInput {
    pub code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResponseError {
    pub code: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response<T>
where
    T: Serialize + DeserializeOwned + 'static,
{
    #[serde(rename = "success")]
    Success {
        #[serde(deserialize_with = "T::deserialize")]
        content: T,
    },
    #[serde(rename = "failed")]
    Failed { error: ResponseError },
}
