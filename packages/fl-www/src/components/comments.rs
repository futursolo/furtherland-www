use crate::prelude::*;

pub(crate) struct BaseComments {
    dispatch: AppDispatch,
}

impl Component for BaseComments {
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
                <h1 class="fl-comment-title">{fl!("comments")}</h1>
                <div class="fl-comment-content-container">
                    <div class="fl-comment-content">{"Coming Soon..."}</div>
                </div>
            </div>
        }
    }
}

impl YieldStyle for BaseComments {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.dispatch.state().theme.current();

        format!(
            r#"
                height: 150px;
                width: 100%;

                display: flex;
                flex-direction: column;
                justify-content: space-around;
                align-items: center;

                .fl-comment-title {{
                    width: 100%;
                }}

                .fl-comment-content {{
                    color: {};
                    font-size: 2rem;
                }}

                .fl-comment-content-container {{
                    flex-grow: 1;

                    display: flex;
                    flex-direction: row;
                    justify-content: space-around;
                    align-items: center;
                }}
            "#,
            theme.colour.text.hint,
        )
        .into()
    }
}

pub(crate) type Comments = WithDispatch<BaseComments>;
