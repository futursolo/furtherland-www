use yew_agent::use_bridge;

use crate::prelude::*;

/// A Provider to hold an instance of worker for the lifetime of the webpage.
#[function_component(WorkerProvider)]
pub(crate) fn worker_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let _ = use_bridge::<agents::markdown::Worker, _>(|_| ());

    html! {
        <>
            {children}
        </>
    }
}
