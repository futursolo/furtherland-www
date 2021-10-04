use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub children: Children,
    #[prop_or(3.0)]
    pub font_size: f32,
}

pub(crate) struct SectionTitle {
    props: Props,
}

impl Component for SectionTitle {
    type Message = ();
    type Properties = Props;

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
            <div class=self.style()><h1>{children}</h1></div>
        }
    }
}

impl YieldStyle for SectionTitle {
    fn style_str(&self) -> Cow<'static, str> {
        format!(
            r#"
                display: flex;
                width: 100%;

                flex-direction: column;
                align-items: flex-start;
                justify-content: flex-start;

                & h1 {{
                    font-size: {}rem;
                }}
            "#,
            self.props.font_size,
        )
        .into()
    }
}
