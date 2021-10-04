use std::rc::Rc;

use crate::prelude::*;
use yew_side_effect::title::TitleProvider as BaseTitleProvider;

use super::ContextProps;

pub(crate) struct RusticTitleProvider {
    props: ContextProps,
    _link: ComponentLink<Self>,
}

impl Component for RusticTitleProvider {
    type Message = ();
    type Properties = ContextProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let children = self.props.children.clone();

        let format_fn = Rc::new(|m: &str| fl!("title", title = m)) as Rc<dyn Fn(&str) -> String>;

        html! {
            <BaseTitleProvider
                default_title=fl!("default-title")
                format_title=format_fn
            >
                {children}
            </BaseTitleProvider>
        }
    }
}

pub(crate) type TitleProvider = WithDispatch<RusticTitleProvider>;
