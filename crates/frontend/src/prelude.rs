pub use std::borrow::Cow;

pub use fl_www_core::prelude::*;
// pub use reqwest;
pub use gloo::utils::{document, window};
pub use wasm_bindgen::JsCast;
pub use wasm_bindgen_futures::spawn_local;
pub use yew::prelude::*;
pub use {crate as frontend, gloo, log, stylist, yew};

pub use crate::browser::BrowserKind;
// pub use crate::client;
pub use crate::utils;
// pub use fl_www_macros as macros;
pub use crate::{hooks, styling};
