// This is the temporary directory until metadata and writings are implemented with fetch.

use rust_embed::RustEmbed;

#[derive(Debug, RustEmbed)]
#[folder = "../../writings"]
pub(crate) struct Writings;
