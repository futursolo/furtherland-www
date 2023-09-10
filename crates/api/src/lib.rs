#![deny(clippy::all)]
#![deny(missing_debug_implementations)]

#[cfg(feature = "resolvable")]
mod resolvers;
mod routines;

use fl_www_core as core;

use self::core::messages;
#[cfg(feature = "resolvable")]
pub use self::resolvers::*;
#[cfg(not(feature = "resolvable"))]
pub use self::routines::*;
