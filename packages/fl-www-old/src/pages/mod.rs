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
pub(crate) enum I18nRoute {
    #[at("/zh/")]
    Chinese,
    #[at("/en/")]
    English,
    #[at("/!")]
    Home,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
}

impl I18nRoute {
    fn render_route(&self) -> Html {
        match self {
            Self::English => AppRoute::current_route().unwrap_or_default().render_route(),
            Self::Chinese => AppRoute::current_route().unwrap_or_default().render_route(),
            Self::Home => {
                let lang = Language::detect();
                html! {<Redirect to={AppRoute::Home { lang }} />}
            }
            Self::PageNotFound => {
                let lang = Language::detect();
                html! {<Redirect to={AppRoute::PageNotFound { lang }} />}
            }
        }
    }
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub(crate) enum AppRoute {
    #[at("/:lang/writings/:slug")]
    Writing { lang: Language, slug: String },
    #[at("/:lang/pages/about!")]
    About { lang: Language },
    #[at("/:lang/page-not-found")]
    PageNotFound { lang: Language },
    #[at("/:lang/!")]
    Home { lang: Language },
    #[not_found]
    #[at("/")]
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
}

impl Default for AppRoute {
    fn default() -> Self {
        let lang = Language::detect();
        Self::PageNotFound { lang }
    }
}

pub(crate) type I18nRouter = Router<I18nRoute>;

#[function_component(AppRouter)]
pub(crate) fn app_router() -> Html {
    html! {
        <I18nRouter
             render={I18nRouter::render(I18nRoute::render_route)}
         />
    }
}
