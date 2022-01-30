use crate::prelude::*;

use once_cell::sync::Lazy;
use reqwest::Url;

mod replies;

static BASE_URL: Lazy<Url> = Lazy::new(|| "http://localhost:9740/".parse().unwrap_throw());

pub use replies::{RepliesQuery, RepliesQueryInput};
