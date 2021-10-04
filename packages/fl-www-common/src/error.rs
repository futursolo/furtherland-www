use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse string as {}, due to: {}", .target_kind, .reason)]
    ParseStr { target_kind: String, reason: String },
}

pub type Result<T> = std::result::Result<T, Error>;
