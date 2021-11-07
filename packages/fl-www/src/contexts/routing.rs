use crate::atoms::LanguageState;
use crate::prelude::*;

use bounce::prelude::*;

pub(crate) fn use_app_route() -> AppRoute {
    use_route::<AppRoute>().unwrap_or_default()
}

#[function_component(RoutingListener)]
pub(crate) fn routing_listener(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let route = use_app_route();

    let set_lang = use_set_bounce_value::<LanguageState>();

    use_effect_with_deps(
        move |_| {
            window().scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.0));

            set_lang(());
            || {}
        },
        route,
    );

    html! {<>{children}</>}
}
