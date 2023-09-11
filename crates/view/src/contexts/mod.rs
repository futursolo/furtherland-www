use bounce::helmet::HelmetBridge as BaseHelmetBridge;
use bounce::BounceRoot;
use yew_router::prelude::BrowserRouter;

use crate::prelude::*;

mod metadata;
mod routing;
mod theme;

pub(crate) use routing::use_app_route;
use routing::RoutingListener;
pub(crate) use theme::use_theme;
use theme::ThemeProvider;

#[function_component(HelmetBridge)]
pub(crate) fn helmet_bridge() -> Html {
    use_language();

    let format_fn = |m: AttrValue| fl!("title", title = m.to_string()).into();

    html! {
        <BaseHelmetBridge
            default_title={fl!("default-title")}
            format_title={format_fn}
        />
    }
}

#[function_component(Providers)]
pub(crate) fn providers(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    html! {
        <BounceRoot>
            <HelmetBridge />
            <BrowserRouter>
                <RoutingListener />
                <ThemeProvider>
                    {children}
                </ThemeProvider>
            </BrowserRouter>
        </BounceRoot>
    }
}
