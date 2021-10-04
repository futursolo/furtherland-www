use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct CodeProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,

    pub children: Children,
}

impl_dispatch_mut!(CodeProps);

pub(crate) struct BaseCode {
    props: CodeProps,
}

impl Component for BaseCode {
    type Message = ();
    type Properties = CodeProps;

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
            <code class=self.style()>
                {children}
            </code>
        }
    }
}

impl YieldStyle for BaseCode {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.props.dispatch.state().theme.current();

        format!(
            r#"
                background-color: {};
                padding-left: 2px;
                padding-right:2px;
                border-radius: 2px;
            "#,
            theme.colour.background.code,
        )
        .into()
    }
}

pub(crate) type Code = WithDispatch<BaseCode>;
