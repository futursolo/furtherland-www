use crate::prelude::*;

use agents::prelude::*;

/// A Provider to hold an instance of worker for the lifetime of the webpage.
#[function_component(WorkerProvider)]
pub(crate) fn worker_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let _: UseBridgeHandle<agents::highlight::Worker> = use_bridge(|_| ());
    let _: UseBridgeHandle<agents::markdown::Worker> = use_bridge(|_| ());

    html! {
        <>
            {children}
        </>
    }
}
