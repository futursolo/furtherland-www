use yew_router::prelude::Route;
use yew_router::router::Router;
use yew_router::service::RouteService;
use yew_router::Switch;

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

#[derive(Switch, Debug, Clone, PartialEq)]
pub(crate) enum I18nRoute {
    #[to = "/zh{*:path}"]
    Chinese(AppRoute),
    #[to = "/en{*:path}"]
    English(AppRoute),
    #[to = "/!"]
    Home,
}

impl I18nRoute {
    fn render_route(self) -> Html {
        match self {
            Self::English(m) => m.render_route(),
            Self::Chinese(m) => m.render_route(),
            Self::Home => {
                let lang = Language::detect();
                html! {<Redirect to=lang.route_i18n(AppRoute::Home) />}
            }
        }
    }

    pub fn current_route() -> Option<Self> {
        let route: Route<()> = RouteService::new().get_route();
        I18nRoute::switch(route)
    }

    pub fn default_route() -> Self {
        let lang = Language::detect();
        lang.route_i18n(AppRoute::PageNotFound)
    }

    pub fn into_app_route(self) -> AppRoute {
        match self {
            Self::English(m) => m,
            Self::Chinese(m) => m,
            Self::Home => AppRoute::Home,
        }
    }
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub(crate) enum AppRoute {
    #[to = "/writings/{slug}"]
    Writing(String),
    #[to = "/pages/about!"]
    About,
    #[to = "/page-not-found"]
    PageNotFound,
    #[to = "/!"]
    Home,
    #[to = "/"]
    Other,
}

impl AppRoute {
    fn render_route(self) -> Html {
        match self {
            Self::Home => {
                html! {<Home />}
            }
            Self::About => {
                html! {<About />}
            }

            Self::Other | Self::PageNotFound => {
                html! {<Other />}
            }
            Self::Writing(s) => html! {<Writing slug=s />},
        }
    }
}

pub(crate) type I18nRouter = Router<I18nRoute>;

pub(crate) struct AppRouter;

impl Component for AppRouter {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_fn = move |route| I18nRoute::render_route(route);

        html! {
        <I18nRouter
             render=I18nRouter::render(render_fn)
             redirect=I18nRouter::redirect(|_route| {
                I18nRoute::default_route()
             })
         />
        }
    }
}
