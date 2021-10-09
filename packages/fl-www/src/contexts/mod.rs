use crate::prelude::*;

mod highlight;
mod i18n;
mod metadata;
mod routing;
mod theme;
mod title;

use highlight::HighlightProvider;
pub(crate) use i18n::use_language;
use i18n::I18nProvider;
pub(crate) use metadata::use_metadata;
use metadata::MetaProvider;
pub(crate) use routing::use_app_route;
use routing::RoutingListener;
pub(crate) use theme::use_theme;
use theme::ThemeProvider;
use title::TitleProvider;

#[function_component(Providers)]
pub(crate) fn providers(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    html! {
        <RoutingListener>
            <I18nProvider>
                <ThemeProvider>
                    <MetaProvider>
                        <TitleProvider>
                            <HighlightProvider>
                                {children}
                            </HighlightProvider>
                        </TitleProvider>
                    </MetaProvider>
                </ThemeProvider>
            </I18nProvider>
        </RoutingListener>
    }
}
