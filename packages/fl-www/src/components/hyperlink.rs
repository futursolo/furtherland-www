use crate::prelude::*;
use styling::Colour;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct HyperlinkProps {
    pub children: Children,
    pub href: String,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub dispatch: AppDispatch,
    #[prop_or_default]
    pub colour: Option<Colour>,

    #[prop_or(false)]
    pub styled: bool,
}

impl_dispatch_mut!(HyperlinkProps);

pub(crate) struct BaseHyperlink {
    props: HyperlinkProps,
}

impl Component for BaseHyperlink {
    type Message = ();
    type Properties = HyperlinkProps;

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
            <a href=self.props.href.clone()
                class=self.style()
                title=self.props.title.as_ref().map(|m| m.to_string())
            >
                {children}
            </a>
        }
    }
}

impl YieldStyle for BaseHyperlink {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.props.dispatch.state().theme.current();

        if self.props.styled {
            format!(
                r#"
                    color: {};
                    transition: color 0.3s;
                    text-decoration: none;

                    &:hover {{
                        color: {};
                        text-decoration: underline;
                    }}
                "#,
                theme.colour.primary, theme.colour.primary_hover,
            )
            .into()
        } else {
            format!(
                r#"
                    text-decoration: none;
                    color: {};

                "#,
                self.props
                    .colour
                    .as_ref()
                    .unwrap_or(&theme.colour.text.primary)
            )
            .into()
        }
    }
}

pub(crate) type Hyperlink = WithDispatch<BaseHyperlink>;
