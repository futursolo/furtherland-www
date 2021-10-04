use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct LayoutProps {
    pub children: Children,
}

pub(crate) struct Layout {
    props: LayoutProps,
}

impl Component for Layout {
    type Message = ();
    type Properties = LayoutProps;

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
            <div class=self.style()>{children}</div>
        }
    }
}

impl YieldStyle for Layout {
    fn style_str(&self) -> Cow<'static, str> {
        r#"
            display: flex;
            width: 100%;
            min-height: 100vh;

            flex-direction: column;
            align-items: center;
            justify-content: flex-start;
        "#
        .into()
    }
}
