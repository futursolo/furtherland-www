use crate::prelude::*;

use hooks::use_event;

pub(crate) fn use_app_route() -> AppRoute {
    let get_current_route = || {
        window()
            .location()
            .pathname()
            .ok()
            .as_ref()
            .and_then(|m| AppRoute::recognize(m))
            .unwrap_or_default()
    };

    let route = use_state(get_current_route);

    log::debug!("Path: {:?}", window().location().pathname());
    log::debug!("Route Changed: {:?}", route);

    let route_clone = route.clone();
    use_event(&window(), "popstate", move |_| {
        route_clone.set(get_current_route());
    });

    let route_clone = route.clone();
    use_effect_with_deps(
        move |_| {
            route_clone.set(get_current_route());
            || {}
        },
        (),
    );

    (*route).clone()
}

#[function_component(RoutingListener)]
pub(crate) fn routing_listener(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    use_event(&window(), "popstate", |_event| {
        window().scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.0));
    });

    html! {<>{children}</>}
}
