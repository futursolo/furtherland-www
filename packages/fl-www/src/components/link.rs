use crate::prelude::*;
use styling::Colour;

type AppAnchor = yew_router::prelude::RouterAnchor<I18nRoute>;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct LinkProps {
    pub children: Children,
    pub to: I18nRoute,
    #[prop_or_default]
    pub dispatch: AppDispatch,
    #[prop_or_default]
    pub colour: Option<Colour>,

    #[prop_or(false)]
    pub styled: bool,
}

impl_dispatch_mut!(LinkProps);

pub(crate) struct BaseLink {
    props: LinkProps,
}

impl Component for BaseLink {
    type Message = ();
    type Properties = LinkProps;

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
        let route = self.props.to.clone();
        let children = self.props.children.clone();
        html! {
            <AppAnchor route=route classes=self.style_class()>{children}</AppAnchor>
        }
    }
}

impl YieldStyle for BaseLink {
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

pub(crate) type Link = WithDispatch<BaseLink>;
