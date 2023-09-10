#![deny(clippy::all)]

mod context;
use fl_www_models::db;
mod encoding;
mod error;
mod prelude;
mod reply;
mod resident;
mod web;

pub use web::WebServer;
