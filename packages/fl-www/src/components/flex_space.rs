use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct FlexSpaceProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or(1)]
    pub scale: u32,
}

pub(crate) struct FlexSpace {
    props: FlexSpaceProps,
}

impl Component for FlexSpace {
    type Message = ();
    type Properties = FlexSpaceProps;

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
        html! {
            <div class=self.style() />
        }
    }
}

impl YieldStyle for FlexSpace {
    fn style_str(&self) -> Cow<'static, str> {
        format!(
            r#"
                flex-grow: {};
            "#,
            self.props.scale
        )
        .into()
    }
}
