use crate::prelude::*;
use store::AppDispatch;

pub(crate) struct BaseContent {
    dispatch: AppDispatch,
}

impl Component for BaseContent {
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
        html! {
            <div class=self.style()>
                <div class="fl-header-home-content-title">{fl!("default-title")}</div>
            </div>
        }
    }
}

impl YieldStyle for BaseContent {
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

pub(crate) type Content = WithDispatch<BaseContent>;
