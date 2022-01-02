use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;

#[derive(PartialEq, Debug, Clone, Eq, Hash, PartialOrd, Ord, Copy)]
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
