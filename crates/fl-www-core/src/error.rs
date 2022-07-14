use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse string as {}, due to: {}", .target_kind, .reason)]
    ParseStr { target_kind: String, reason: String },
    #[error("Failed to serialise / deserialise JSON")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
