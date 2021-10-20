use crate::prelude::*;

use hooks::use_event;

pub(crate) fn use_app_route() -> AppRoute {
    use_route::<AppRoute>().unwrap_or_default()
}

#[function_component(RoutingListener)]
pub(crate) fn routing_listener(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    use_event(&window(), "popstate", |_event| {
        window().scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.0));
    });

    html! {<>{children}</>}
}
