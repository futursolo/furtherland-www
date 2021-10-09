use std::rc::Rc;

use crate::prelude::*;

use common::client::ClientProvider as BaseClientProvider;

/// A Provider to hold an instance of worker for the lifetime of the webpage.
#[function_component(ClientProvider)]
pub(crate) fn client_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let client = (*use_state(|| Rc::new(reqwest::Client::new()))).clone();

    html! {
        <BaseClientProvider client={client}>
            {children}
        </BaseClientProvider>
    }
}
