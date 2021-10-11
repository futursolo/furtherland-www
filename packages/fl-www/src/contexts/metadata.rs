use std::rc::Rc;

use crate::prelude::*;
use yew_query::{use_query, Request};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MetadataState {
    value: Option<Rc<Metadata>>,
}

impl Default for MetadataState {
    fn default() -> Self {
        Self { value: None }
    }
}

pub(crate) fn use_metadata() -> Option<Rc<Metadata>> {
    use_context::<MetadataState>().unwrap().value
}

#[function_component(MetaProvider)]
pub(crate) fn meta_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();
    let error = use_error_state();

    // let base_url = use_base_url();

    // let error_clone = error.clone();
    let meta: Option<Rc<Metadata>> =
        match use_query(|| Request::builder().url("/metadata.json").build()).result() {
            Some(Ok(m)) => Some(m.data()),
            Some(Err(_)) => {
                error.set(ErrorKind::Server);

                None
            }
            _ => None,
        };

    let state = MetadataState { value: meta };

    html! {
        <ContextProvider<MetadataState> context={state}>
            {children}
        </ContextProvider<MetadataState>>
    }
}
