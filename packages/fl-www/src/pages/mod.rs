use yew_router::prelude::*;

use crate::prelude::*;

mod about;
mod home;
mod other;
mod writing;

use about::About;
use home::Home;
use other::Other;
use writing::Writing;

use components::Redirect;
use i18n::Language;

#[derive(Routable, Debug, Clone, PartialEq)]
pub(crate) enum AppRoute {
    #[at("/:lang/writings/:slug")]
    Writing { lang: Language, slug: String },
    #[at("/:lang/pages/about")]
    About { lang: Language },
    #[at("/:lang/page-not-found")]
    PageNotFound { lang: Language },
    #[at("/:lang/")]
    Home { lang: Language },
    #[not_found]
    #[at("/page-not-found")]
    Other,
}

impl AppRoute {
    fn render_route(&self) -> Html {
        match self {
            Self::Home { .. } => {
                html! {<Home />}
            }
            Self::About { .. } => {
                html! {<About />}
            }

            Self::Other | Self::PageNotFound { .. } => {
                html! {<Other />}
            }
            Self::Writing { slug, .. } => html! {<Writing slug={slug.clone()} />},
        }
    }

    pub fn with_lang(self, lang: Language) -> Self {
        match self {
            Self::Home { .. } => Self::Home { lang },
            Self::About { .. } => Self::About { lang },

            Self::Other => Self::Other,

            Self::PageNotFound { .. } => Self::PageNotFound { lang },
            Self::Writing { slug, .. } => Self::Writing { slug, lang },
        }
    }

    pub fn lang(&self) -> Option<Language> {
        match self {
            Self::Home { lang, .. } => Some(*lang),
            Self::About { lang, .. } => Some(*lang),

            Self::PageNotFound { lang, .. } => Some(*lang),
            Self::Writing { lang, .. } => Some(*lang),
            _ => None,
        }
    }
}

impl Default for AppRoute {
    fn default() -> Self {
        let lang = Language::detect();
        Self::PageNotFound { lang }
    }
}

#[function_component(AppRouter)]
pub(crate) fn app_router() -> Html {
    html! {
        <Router<AppRoute>
             render={Router::render(AppRoute::render_route)}
         />
    }
}
