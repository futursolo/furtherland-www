use yew_router::prelude::*;

use crate::prelude::*;
use bounce::helmet::Helmet;

mod home;
mod loading;
mod other;
mod page;
mod residents;
mod writing;

use home::Home;
use loading::Loading;
use other::Other;
use page::Page;
use writing::Writing;

use components::Redirect;

#[function_component(HomeRedirect)]
fn home_redirect() -> Html {
    let lang = use_language();
    let home_route = AppRoute::Home { lang };
    html! {<Redirect to={home_route} />}
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub(crate) enum AppRoute {
    #[at("/:lang/writings/:slug")]
    Writing { lang: Language, slug: String },
    #[at("/:lang/pages/:slug")]
    Page { lang: Language, slug: String },
    #[at("/:lang/page-not-found")]
    PageNotFound { lang: Language },
    #[at("/:lang/")]
    Home { lang: Language },
    #[at("/residents/github/continue")]
    ResidentGitHubContinue,
    #[at("/loading")]
    Loading,
    #[at("/")]
    HomeRedirect,
    #[at("/page-not-found")]
    #[not_found]
    Other,
}

impl AppRoute {
    fn render_route(&self) -> Html {
        match self {
            Self::Home { .. } => {
                html! {<Home />}
            }
            Self::HomeRedirect => {
                html! {<HomeRedirect />}
            }

            Self::Loading => {
                html! {<Loading />}
            }

            Self::ResidentGitHubContinue => {
                html! { <residents::github::OauthContinue /> }
            }

            Self::Other | Self::PageNotFound { .. } => {
                html! {<Other />}
            }
            Self::Writing { slug, .. } => html! {<Writing slug={slug.clone()} />},
            Self::Page { slug, .. } => html! {<Page slug={slug.clone()} />},
        }
    }

    pub fn with_lang(self, lang: Language) -> Self {
        match self {
            Self::Home { .. } => Self::Home { lang },
            Self::Loading => Self::Loading,

            Self::HomeRedirect => Self::HomeRedirect,
            Self::Other => Self::Other,
            Self::ResidentGitHubContinue => Self::ResidentGitHubContinue,

            Self::PageNotFound { .. } => Self::PageNotFound { lang },
            Self::Writing { slug, .. } => Self::Writing { slug, lang },
            Self::Page { slug, .. } => Self::Page { slug, lang },
        }
    }

    pub fn lang(&self) -> Option<Language> {
        match self {
            Self::Home { lang } => Some(*lang),
            Self::PageNotFound { lang, .. } => Some(*lang),
            Self::Writing { lang, .. } => Some(*lang),
            Self::Page { lang, .. } => Some(*lang),

            Self::HomeRedirect | Self::Other | Self::Loading | Self::ResidentGitHubContinue => None,
        }
    }
}

#[function_component(AppRouter)]
pub(crate) fn app_router() -> Html {
    let lang = use_language();

    let feed_url = match lang {
        Language::Chinese => "/feed-zh.xml",
        Language::English => "/feed-en.xml",
    };

    html! {
        <>
            <Helmet>
                <link rel="alternate" href={feed_url} type_="application/atom+xml" />
            </Helmet>
            <Switch<AppRoute>
                 render={Switch::render(AppRoute::render_route)}
             />
        </>
    }
}
