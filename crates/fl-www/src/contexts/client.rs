use std::rc::Rc;

use yew_query::ClientProvider as BaseClientProvider;

use crate::prelude::*;

/// A Provider to hold an instance of worker for the lifetime of the webpage.
#[function_component(ClientProvider)]
pub(crate) fn client_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let client = (*use_state(|| Rc::new(yew_query::Client::default()))).clone();

    html! {
        <BaseClientProvider client={client}>
            {children}
        </BaseClientProvider>
    }
}
