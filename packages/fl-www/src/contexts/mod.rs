use crate::prelude::*;

mod common;
mod global_style;
mod i18n;
mod routing;
mod theme;
mod title;

use common::ContextProps;
use i18n::I18nProvider;
use routing::RoutingListener;
use theme::ThemeProvider;
use title::TitleProvider;

pub(crate) struct BaseProviders {
    props: ContextProps,
}

impl Component for BaseProviders {
    type Message = ();
    type Properties = ContextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let children = self.props.children.clone();

        html! {
            <RoutingListener>
                <I18nProvider>
                    <ThemeProvider>
                        <TitleProvider>
                            {children}
                        </TitleProvider>
                    </ThemeProvider>
                </I18nProvider>
            </RoutingListener>
        }
    }
}

pub(crate) type Providers = WithDispatch<BaseProviders>;
