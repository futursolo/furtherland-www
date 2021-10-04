use crate::prelude::*;

use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    route::Route,
};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct RedirectProps {
    pub to: I18nRoute,
}

pub(crate) struct Redirect {
    props: RedirectProps,

    router: RouteAgentDispatcher<()>,
}

impl Component for Redirect {
    type Message = ();
    type Properties = RedirectProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        Self { props, router }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let route = Route::from(self.props.to.clone());
            self.router.send(RouteRequest::ChangeRoute(route));
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        Html::default()
    }
}
