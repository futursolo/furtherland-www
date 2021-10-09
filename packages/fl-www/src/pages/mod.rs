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

#[function_component(HomeRedirect)]
fn home_redirect() -> Html {
    let lang = use_language();
    let home_route = match lang {
        Language::Chinese => AppRoute::HomeZh,
        Language::English => AppRoute::HomeEn,
    };

    html! {<Redirect to={home_route} />}
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub(crate) enum AppRoute {
    #[at("/:lang/writings/:slug")]
    Writing { lang: Language, slug: String },
    #[at("/:lang/pages/about")]
    About { lang: Language },
    #[at("/:lang/page-not-found")]
    PageNotFound { lang: Language },
    #[at("/en")]
    HomeEn,
    #[at("/zh")]
    HomeZh,
    #[at("/")]
    HomeRedirect,
    #[at("/404")]
    #[not_found]
    Other,
}

impl AppRoute {
    fn render_route(&self) -> Html {
        match self {
            Self::HomeEn | Self::HomeZh { .. } => {
                html! {<Home />}
            }
            Self::HomeRedirect => {
                html! {<HomeRedirect />}
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
            Self::HomeEn | Self::HomeZh => match lang {
                Language::Chinese => Self::HomeZh,
                Language::English => Self::HomeEn,
            },
            Self::About { .. } => Self::About { lang },

            Self::HomeRedirect => Self::HomeRedirect,
            Self::Other => Self::Other,

            Self::PageNotFound { .. } => Self::PageNotFound { lang },
            Self::Writing { slug, .. } => Self::Writing { slug, lang },
        }
    }

    pub fn lang(&self) -> Option<Language> {
        match self {
            Self::HomeEn => Some(Language::English),
            Self::HomeZh => Some(Language::Chinese),
            Self::About { lang, .. } => Some(*lang),

            Self::PageNotFound { lang, .. } => Some(*lang),
            Self::Writing { lang, .. } => Some(*lang),

            Self::HomeRedirect => None,
            Self::Other => None,
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
    log::debug!("{:?}", AppRoute::not_found_route());

    html! {
        <Router<AppRoute>
             render={Router::render(AppRoute::render_route)}
         />
    }
}
