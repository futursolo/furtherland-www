#![deny(clippy::all)]

mod context;
pub mod db;
mod encoding;
mod error;
mod prelude;
mod reply;
mod resident;
mod web;

pub use web::WebServer;
