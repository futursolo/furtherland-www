use bounce::prelude::*;

use crate::atoms::LanguageState;
use crate::prelude::*;

pub(crate) fn use_app_route() -> AppRoute {
    use_route::<AppRoute>().unwrap_or_default()
}

#[function_component(RoutingListener)]
pub(crate) fn routing_listener() -> Html {
    let route = use_app_route();

    let set_lang = use_slice_dispatch::<LanguageState>();

    use_effect_with_deps(
        move |_| {
            window().scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.0));

            set_lang(());
            || {}
        },
        route,
    );

    Html::default()
}
