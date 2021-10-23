use crate::prelude::*;

use yew_router::prelude::BrowserRouter;

mod client;
mod error;
mod helmet;
mod i18n;
mod metadata;
mod routing;
mod theme;
mod title;
// mod utils;
mod worker;

use client::ClientProvider;
use error::ErrorProvider;
pub(crate) use error::{use_error_state, ErrorKind};
use helmet::HelmetProvider;
pub(crate) use helmet::{Meta, MetaLink, Script};
pub(crate) use i18n::use_language;
use i18n::I18nProvider;
pub(crate) use metadata::use_metadata;
use metadata::MetaProvider;
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
        <ErrorProvider>
            <BrowserRouter>
                <RoutingListener>
                    <I18nProvider>
                        <HelmetProvider>
                            <ThemeProvider>
                                <MetaProvider>
                                    <TitleProvider>
                                        <WorkerProvider>
                                            <ClientProvider>
                                                {children}
                                            </ClientProvider>
                                        </WorkerProvider>
                                    </TitleProvider>
                                </MetaProvider>
                            </ThemeProvider>
                        </HelmetProvider>
                    </I18nProvider>
                </RoutingListener>
            </BrowserRouter>
        </ErrorProvider>
    }
}
