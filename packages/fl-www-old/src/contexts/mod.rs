use crate::prelude::*;

mod i18n;
mod metadata;
mod routing;
mod theme;
mod title;

pub(crate) use i18n::use_language;
use i18n::I18nProvider;
pub(crate) use metadata::use_metadata;
use metadata::MetaProvider;
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
                            {children}
                        </TitleProvider>
                    </MetaProvider>
                </ThemeProvider>
            </I18nProvider>
        </RoutingListener>
    }
}
