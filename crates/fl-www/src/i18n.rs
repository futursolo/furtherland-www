use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::{LanguageLoader, WebLanguageRequester};
use once_cell::sync::Lazy;
use unic_langid::LanguageIdentifier;
use yew_router::prelude::Routable;

use crate::prelude::*;

mod l10n_embed {
    // We only want to suppress this limit for Localization.
    #![allow(non_upper_case_globals)]

    use rust_embed::RustEmbed;

    #[derive(Debug, RustEmbed)]
    #[folder = "../../i18n"] // path to the compiled localization resources
    pub(super) struct Localizations;
}

use l10n_embed::Localizations;

pub(crate) trait LanguageExt {
    fn detect() -> Self;
    fn to_lang_id(self) -> LanguageIdentifier;
    fn activate(&self);
}

impl LanguageExt for Language {
    fn detect() -> Self {
        if let Some(m) = window()
            .location()
            .pathname()
            .ok()
            .and_then(|m| AppRoute::recognize(&m))
            .and_then(|m| m.lang())
        {
            return m;
        }

        let requested_langs = WebLanguageRequester::requested_languages();

        for lang in requested_langs.iter() {
            if let Some(m) = lang.to_string().split('-').next() {
                if let Ok(m) = m.parse() {
                    return m;
                }
            }
        }

        Self::English
    }

    fn to_lang_id(self) -> LanguageIdentifier {
        self.as_str().parse().expect("Failed to parse.")
    }

    fn activate(&self) {
        log::debug!("Activating: {:?}", self);

        if LOADER.current_language() != self.to_lang_id() {
            let _result = i18n_embed::select(&*LOADER, &Localizations, &[self.to_lang_id()]);
        }
    }
}

pub(crate) static LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader = fluent_language_loader!();
    let _result = i18n_embed::select(&loader, &Localizations, &[Language::English.to_lang_id()]);
    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $message_id, $($args), *)
    }};
}
