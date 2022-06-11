use std::rc::Rc;

use crate::prelude::*;

use bounce::helmet::HelmetBridge as BaseHelmetBridge;
use bounce::BounceRoot;
use yew_router::prelude::BrowserRouter;

mod client;
mod metadata;
mod routing;
mod theme;
mod worker;

use client::ClientProvider;
pub(crate) use metadata::use_metadata;
pub(crate) use routing::use_app_route;
use routing::RoutingListener;
pub(crate) use theme::use_theme;
use theme::ThemeProvider;
use worker::WorkerProvider;

#[function_component(HelmetBridge)]
pub(crate) fn helmet_bridge() -> Html {
    use_language();

    let format_fn = Rc::new(|m: &str| fl!("title", title = m)) as Rc<dyn Fn(&str) -> String>;

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
                    <WorkerProvider>
                        <ClientProvider>
                            {children}
                        </ClientProvider>
                    </WorkerProvider>
                </ThemeProvider>
            </BrowserRouter>
        </BounceRoot>
    }
}
