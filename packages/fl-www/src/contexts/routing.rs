use yew_router::agent::RouteAgentBridge;

use crate::prelude::*;
use store::Action;

use super::ContextProps;

#[derive(Debug, PartialEq)]
pub(crate) enum Msg {
    RouteUpdated,
}

pub(crate) struct BaseRoutingListener {
    props: ContextProps,
    _route_bridge: RouteAgentBridge,
    current_route: Option<AppRoute>,
}

impl Component for BaseRoutingListener {
    type Message = Msg;
    type Properties = ContextProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Self::Message::RouteUpdated);
        Self {
            props,
            _route_bridge: RouteAgentBridge::new(callback),
            current_route: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.props.dispatch.send(Action::RouteUpdated);
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        let next_route = I18nRoute::current_route().map(|m| m.into_app_route());

        // Restore Scroll.
        if self.current_route != next_route {
            window().scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.0));
        }

        self.current_route = next_route;
        self.props.dispatch.send(Action::RouteUpdated);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let children = self.props.children.clone();
        html! {<>{children}</>}
    }
}

pub(crate) type RoutingListener = WithDispatch<BaseRoutingListener>;
