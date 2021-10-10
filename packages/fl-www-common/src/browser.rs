use once_cell::sync::Lazy;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrowserKind {
    Safari,
    Firefox,
    Chrome,
    Other,
}

impl BrowserKind {
    pub fn detect() -> BrowserKind {
        static KIND: Lazy<BrowserKind> = Lazy::new(|| {
            window()
                .navigator()
                .user_agent()
                .map(|m| {
                    if m.contains("Chrome") {
                        BrowserKind::Chrome
                    } else if m.contains("Firefox") {
                        BrowserKind::Firefox
                    } else if m.contains("Edge") {
                        BrowserKind::Other
                    } else if m.contains("Safari") {
                        BrowserKind::Safari
                    } else {
                        BrowserKind::Other
                    }
                })
                .unwrap_or(BrowserKind::Other)
        });

        *KIND
    }
}
