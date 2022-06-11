use std::rc::Rc;

use bounce::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

static STORAGE_KEY: &str = "fl_resident_token";

#[derive(Atom, PartialEq, Serialize, Deserialize)]
#[bounce(observed)]
pub(crate) struct TokenState {
    pub inner: Option<String>,
}

impl Default for TokenState {
    fn default() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or(Self { inner: None })
    }
}

impl Observed for TokenState {
    fn changed(self: Rc<Self>) {
        LocalStorage::set(STORAGE_KEY, &self).expect_throw("failed to set theme.");
    }
}
