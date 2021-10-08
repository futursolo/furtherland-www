use std::fmt;
use std::str::FromStr;

use i18n_embed::LanguageLoader;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    WebLanguageRequester,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;
use yew_router::prelude::Routable;

use crate::prelude::*;

#[derive(Debug, RustEmbed)]
#[folder = "../../i18n"] // path to the compiled localization resources
struct Localizations;

#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn detect() -> Self {
        log::debug!("{:?}", I18nRoute::current_route());

        if let Some(m) = I18nRoute::current_route() {
            match m {
                I18nRoute::English => return Self::English,
                I18nRoute::Chinese => return Self::Chinese,
                _ => (),
            }
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

    pub fn as_str(&self) -> &str {
        match self {
            Self::Chinese => "zh",
            Self::English => "en",
        }
    }

    pub fn to_lang_id(self) -> LanguageIdentifier {
        self.as_str().parse().expect("Failed to parse.")
    }

    pub fn activate(&self) {
        log::debug!("Activating: {:?}", self);

        if (&*LOADER).current_language() != self.to_lang_id() {
            let _result = i18n_embed::select(&*LOADER, &Localizations, &[self.to_lang_id()]);
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "zh" => Ok(Self::Chinese),
            "en" => Ok(Self::English),
            _ => Err(Error::ParseStr {
                target_kind: "Language".to_string(),
                reason: "Value is not valid.".to_string(),
            }),
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
