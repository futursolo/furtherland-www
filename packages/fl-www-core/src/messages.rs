use crate::object_id::ObjectId;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResidencyStatus {
    // Disabled = -1,
    // Pending = 0,
    Resident = 10,
    Master = 100,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Replies {
    pub replies: Vec<Reply>,
    // None if is last item has been reached.
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reply {
    pub id: ObjectId,
    pub slug: String,
    pub resident: Option<Resident>,
    pub content: String,
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
pub struct ResponseError {
    pub code: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response<T>
where
    T: Serialize + DeserializeOwned + 'static,
{
    Success {
        #[serde(deserialize_with = "T::deserialize")]
        content: T,
    },
    Failed {
        error: ResponseError,
    },
}
