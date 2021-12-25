use crate::object_id::ObjectId;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.id == 11693215
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comments {
    pub comments: Vec<Comment>,
    // None if is last item has been reached.
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: ObjectId,
    pub slug: String,
    pub user: Option<User>,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentInput {
    pub content: String,
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
