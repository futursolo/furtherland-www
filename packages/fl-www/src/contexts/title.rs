use std::rc::Rc;

use crate::prelude::*;
use yew_side_effect::title::TitleProvider as BaseTitleProvider;

#[function_component(TitleProvider)]
pub(crate) fn title_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let format_fn = Rc::new(|m: &str| fl!("title", title = m)) as Rc<dyn Fn(&str) -> String>;

    html! {
        <BaseTitleProvider
            default_title={fl!("default-title")}
            format_title={format_fn}
        >
            {children}
        </BaseTitleProvider>
    }
}
