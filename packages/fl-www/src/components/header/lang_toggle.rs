use yew_feather::globe::Globe;

use crate::prelude::*;
use components::Link;
use store::AppDispatch;
use styling::Colour;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct LangToggleProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,
    pub colour: Colour,
}

impl_dispatch_mut!(LangToggleProps);

pub(crate) struct BaseLangToggle {
    props: LangToggleProps,
}

impl Component for BaseLangToggle {
    type Message = ();
    type Properties = LangToggleProps;

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
        let lang = &self.props.dispatch.state().i18n.lang;
        let route = I18nRoute::current_route()
            .and_then(|m| match m {
                I18nRoute::English(m) => Some(I18nRoute::Chinese(m)),
                I18nRoute::Chinese(m) => Some(I18nRoute::English(m)),
                I18nRoute::Home => None,
            })
            .unwrap_or_else(|| lang.route_i18n(AppRoute::Home));

        html! {
            <Link to=route>
                <div class=self.style()>
                    <Globe size=24 />
                </div>
            </Link>
        }
    }
}

impl YieldStyle for BaseLangToggle {
    fn style_str(&self) -> Cow<'static, str> {
        format!(
            r#"
                height: 60px;
                width: 60px;
                color: {};

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;
                cursor: pointer;
            "#,
            self.props.colour
        )
        .into()
    }
}

pub(crate) type LangToggle = WithDispatch<BaseLangToggle>;
