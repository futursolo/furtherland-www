use crate::prelude::*;

use i18n::Language;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct I18nState {
    pub lang: Language,
}

impl Default for I18nState {
    fn default() -> Self {
        Self {
            lang: Language::detect(),
        }
    }
}

impl I18nState {
    pub fn sync(&mut self) {
        let lang = Language::detect();
        let changed = self.lang != lang;

        if changed {
            self.lang = lang;
        }

        self.init();
    }

    pub fn init(&mut self) {
        self.lang.activate()
    }
}
