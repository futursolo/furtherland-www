use crate::prelude::*;

use yew_agent::Bridged;

/// A Provider to hold an instance of worker for the lifetime of the webpage.
#[function_component(HighlightProvider)]
pub(crate) fn highlight_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    use_state(|| agents::highlight::Worker::bridge(Callback::from(move |_| ())));

    html! {
        <>
            {children}
        </>
    }
}
