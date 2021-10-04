use gloo::events::EventListener;

use crate::prelude::*;
use store::Action;

use super::global_style::GlobalStyle;
use super::ContextProps;

pub(crate) struct BaseThemeProvider {
    props: ContextProps,
    listener: EventListener,
}

impl Component for BaseThemeProvider {
    type Message = ();
    type Properties = ContextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let dispatch = props.dispatch.clone();

        let listener = EventListener::new(&window(), "storage", move |_event| {
            dispatch.clone().send(Action::ThemeUpdated);
        });

        Self { props, listener }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let sync_fn = self.listener.callback();

            if let Some(m) = window()
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .and_then(|m| m)
            {
                m.add_listener_with_opt_callback(Some(sync_fn.as_ref().unchecked_ref()))
                    .expect("Failed to listen to colour scheme changes.");
            }
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let children = self.props.children.clone();
        html! {<GlobalStyle>{children}</GlobalStyle>}
    }
}

pub(crate) type ThemeProvider = WithDispatch<BaseThemeProvider>;
