use web_sys::{ScrollBehavior, ScrollIntoViewOptions};
use yew_feather::chevron_down::ChevronDown;

use crate::prelude::*;
use store::AppDispatch;

pub(crate) struct BaseHomeContent {
    dispatch: AppDispatch,
}

impl Component for BaseHomeContent {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let scroll_to_main = Callback::from(|_| {
            if let Some(m) = document().query_selector("nav").ok().flatten() {
                m.scroll_into_view_with_scroll_into_view_options(
                    ScrollIntoViewOptions::new().behavior(ScrollBehavior::Smooth),
                );
            }
        });

        html! {
            <div class=self.style()>
                <div class="fl-header-home-content-before"></div>
                <div class="fl-header-home-content-title">{fl!("default-title")}</div>
                <div class="fl-header-home-content-after">
                    <div class="fl-header-home-content-go-to-main" onclick=scroll_to_main><ChevronDown size=100 /></div>
                </div>
            </div>
        }
    }
}

impl YieldStyle for BaseHomeContent {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.dispatch.state().theme.current();
        format!(
            r#"
                height: 1px;
                width: 100%;

                flex-grow: 1;

                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: space-around;

                & .fl-header-home-content-before {{
                    height: 150px;
                }}

                & .fl-header-home-content-go-to-main {{
                    cursor: pointer;
                    box-sizing: border-box;
                }}

                & .fl-header-home-content-after {{
                    height: 150px;
                }}

                & .fl-header-home-content-title {{
                    font-size: 5rem;
                }}

                {} {{
                    & .fl-header-home-content-title {{
                        font-size: 4rem;
                    }}
                }}

                {} {{
                    & .fl-header-home-content-title {{
                        font-size: 3rem;
                    }}
                }}

                {} {{
                    & .fl-header-home-content-title {{
                        font-size: 2rem;
                    }}
                }}

            "#,
            theme.breakpoint.lg.down(),
            theme.breakpoint.md.down(),
            theme.breakpoint.sm.down(),
        )
        .into()
    }
}

pub(crate) type HomeContent = WithDispatch<BaseHomeContent>;
