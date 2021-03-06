use std::convert::{TryFrom, TryInto};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use gloo::timers::future::TimeoutFuture;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

// use reqwest::Url;
use crate::prelude::*;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct Id(u64);

impl Default for Id {
    fn default() -> Self {
        static CTR: Lazy<AtomicU64> = Lazy::new(AtomicU64::default);

        Self(CTR.fetch_add(1, Ordering::SeqCst))
    }
}

impl Id {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

pub fn get_scroll_y() -> Option<u32> {
    let pos = document().document_element()?.scroll_top();

    if pos > 0 {
        let pos = pos.try_into().ok()?;
        return Some(pos);
    }

    let pos = document().body()?.scroll_top();

    if pos >= 0 {
        let pos = pos.try_into().ok()?;
        return Some(pos);
    }

    None
}

pub fn get_viewport_height() -> u64 {
    window()
        .inner_height()
        .ok()
        .and_then(|m| m.as_f64())
        .and_then(|m| (m as i64).try_into().ok())
        .unwrap_or_default()
}

// pub fn get_base_url() -> Option<Url> {
//     window()
//         .location()
//         .href()
//         .ok()
//         .and_then(|m| Url::parse(&m).ok())
// }

pub fn is_ssr() -> bool {
    window().location().port().unwrap_or_else(|_| "".into()) == "9742"
}

pub fn sleep(dur: Duration) -> TimeoutFuture {
    let millis = u32::try_from(dur.as_millis())
        .expect_throw("failed to cast the duration into a u32 with Duration::as_millis.");

    TimeoutFuture::new(millis)
}
