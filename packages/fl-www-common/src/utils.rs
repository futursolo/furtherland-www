use std::convert::TryInto;
use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;

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
