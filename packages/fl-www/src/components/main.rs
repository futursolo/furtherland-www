use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct MainProps {
    #[prop_or_default]
    dispatch: AppDispatch,
    pub children: Children,
}

impl_dispatch_mut!(MainProps);

pub(crate) struct BaseMain {
    props: MainProps,
}

impl Component for BaseMain {
    type Message = ();
    type Properties = MainProps;

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
            <main class=self.style()><div class="fl-main-container">{children}</div></main>
        }
    }
}

impl YieldStyle for BaseMain {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.props.dispatch.state().theme.current();

        let min_height_str = if let Some(m) = I18nRoute::current_route() {
            let m = m.into_app_route();

            match m {
                AppRoute::Home => "min-height: calc(100vh - 160px);",
                _ => "",
            }
        } else {
            ""
        };

        format!(
            r#"
                display: flex;
                width: 100%;
                flex-grow: 1;
                padding-top: 20px;
                padding-bottom: 20px;
                {}

                flex-direction: column;
                align-items: center;
                justify-content: flex-start;

                & .fl-main-container {{
                    width: calc(100% - 40px);
                    max-width: {};
                }}
            "#,
            min_height_str,
            theme.breakpoint.md.width_str()
        )
        .into()
    }
}

pub(crate) type Main = WithDispatch<BaseMain>;
