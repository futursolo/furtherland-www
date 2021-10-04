use chrono::{Datelike, Local};
use gloo::events::EventListener;

use crate::prelude::*;

use components::FlexSpace;

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Msg {
    Resized,
}

pub(crate) struct BaseFooter {
    dispatch: AppDispatch,
    _resize_listener: EventListener,
    _orientation_listener: EventListener,
}

impl Component for BaseFooter {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_clone = link.clone();
        let window = window();

        let _resize_listener = EventListener::new(&window, "resize", move |_e| {
            link_clone.clone().send_message(Msg::Resized);
        });

        let _orientation_listener = EventListener::new(&window, "resize", move |_e| {
            link.clone().send_message(Msg::Resized);
        });

        Self {
            dispatch,
            _resize_listener,
            _orientation_listener,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let now = Local::now();

        let year = now.year();

        html! {
            <footer class=self.style()>
                <div class="fl-footer-container">
                    <div>
                        {format!("© {} ", year)}
                        {fl!("default-title")}
                    </div>
                    {if !self.is_vertical() { html!{<FlexSpace />} } else { html!{} }}
                    <div class="fl-footer-copy">
                        {fl!("footer-copy")}
                    </div>
                </div>
            </footer>
        }
    }
}

impl BaseFooter {
    fn is_vertical(&self) -> bool {
        let theme = self.dispatch.state().theme.current();

        theme.breakpoint.md.matches_down()
    }
}

impl YieldStyle for BaseFooter {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.dispatch.state().theme.current();

        format!(
            r#"
                height: 100px;
                width: 100%;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
                align-items: center;

                box-sizing: border-box;

                .fl-footer-container {{
                    max-width: calc({} - 40px);
                    width: 100%;

                    display: flex;
                    flex-direction: row;
                    justify-content: space-around;
                    align-items: center;
                }}

                .fl-footer-copy {{
                    color: {};
                }}

                {} {{
                    .fl-footer-container {{
                        flex-direction: column;
                    }}
                }}
            "#,
            theme.breakpoint.md.width_str(),
            theme.colour.text.secondary,
            theme.breakpoint.md.down()
        )
        .into()
    }
}

pub(crate) type Footer = WithDispatch<BaseFooter>;
