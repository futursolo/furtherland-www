#![deny(clippy::all)]
#![deny(missing_debug_implementations)]

#[cfg(feature = "resolvable")]
mod resolvers;
mod routines;

use fl_www_core::messages;
use fl_www_models::db;
#[cfg(feature = "resolvable")]
pub use resolvers::*;
#[cfg(not(feature = "resolvable"))]
pub use routines::*;
