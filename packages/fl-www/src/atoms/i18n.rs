use std::rc::Rc;

use bounce::*;

use crate::prelude::*;
use crate::utils::Id;

#[derive(Debug, Clone, PartialEq, Slice)]
pub(crate) struct LanguageState {
    lang: Language,
    id: Id,
}
impl LanguageState {
    fn sync_tags(&self) {
        let html_element = document()
            .document_element()
            .expect("Failed to get <html /> element.");

        html_element
            .set_attribute("lang", self.lang.as_str())
            .expect("Failed to set language.");

        self.lang.activate();
    }
}

impl Default for LanguageState {
    fn default() -> Self {
        let self_ = Self {
            lang: Language::detect(),
            id: Id::new(),
        };

        self_.sync_tags();

        self_
    }
}

impl Reducible for LanguageState {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        let language = Language::detect();

        if language == self.lang {
            return self;
        }

        let self_ = Self {
            lang: language,
            id: Id::new(),
        };

        self_.sync_tags();

        self_.into()
    }
}

pub(crate) fn use_language() -> Language {
    use_slice_value::<LanguageState>().lang
}
