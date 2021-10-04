use crate::prelude::*;

use styling::{Theme, ThemeKind};

static STORAGE_KEY: &str = "fl_theme";

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ThemeState {
    pub kind: Option<ThemeKind>,
}

impl Default for ThemeState {
    fn default() -> Self {
        if let Some(stor) = window().local_storage().ok().and_then(|m| m) {
            if let Some(kind) = stor
                .get_item(STORAGE_KEY)
                .ok()
                .and_then(|m| m)
                .and_then(|m| m.parse::<ThemeKind>().ok())
            {
                return Self { kind: Some(kind) };
            }

            stor.remove_item(STORAGE_KEY)
                .expect("Failed to remove item.");
        }

        Self { kind: None }
    }
}

impl ThemeState {
    pub fn set_theme_kind(&mut self, kind: Option<ThemeKind>) -> ShouldNotify {
        let changed = Some(self.current_kind()) != kind;

        self.kind = kind;

        if let Some(stor) = window().local_storage().ok().and_then(|m| m) {
            if changed {
                if let Some(ref m) = self.kind {
                    stor.set_item(STORAGE_KEY, m.as_str())
                        .expect("Failed to set item.");
                } else {
                    stor.remove_item(STORAGE_KEY)
                        .expect("Failed to remove item.");
                }
            }
        }

        changed
    }

    pub fn sync(&mut self) {
        let new_self = Self::default();

        self.kind = new_self.kind;
    }

    pub fn current_kind(&self) -> ThemeKind {
        match &self.kind {
            Some(m) => m.clone(),
            None => ThemeKind::current(),
        }
    }

    pub fn current(&self) -> &'static Theme {
        Theme::from_kind(&self.current_kind())
    }
}
