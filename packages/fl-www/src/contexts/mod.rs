use crate::prelude::*;

use bounce::BounceRoot;
use yew_router::prelude::BrowserRouter;

mod client;
mod helmet;
mod metadata;
mod routing;
mod theme;
mod title;
mod worker;

use client::ClientProvider;
use helmet::HelmetProvider;
pub(crate) use helmet::{Meta, MetaLink, Script};
pub(crate) use metadata::use_metadata;
pub(crate) use routing::use_app_route;
use routing::RoutingListener;
pub(crate) use theme::use_theme;
use theme::ThemeProvider;
use title::TitleProvider;
use worker::WorkerProvider;

#[function_component(Providers)]
pub(crate) fn providers(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    html! {
        <BounceRoot>
            <BrowserRouter>
                <RoutingListener>
                    <HelmetProvider>
                        <ThemeProvider>
                            <TitleProvider>
                                <WorkerProvider>
                                    <ClientProvider>
                                        {children}
                                    </ClientProvider>
                                </WorkerProvider>
                            </TitleProvider>
                        </ThemeProvider>
                    </HelmetProvider>
                </RoutingListener>
            </BrowserRouter>
        </BounceRoot>
    }
}
