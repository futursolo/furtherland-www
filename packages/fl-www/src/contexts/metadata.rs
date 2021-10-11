use std::rc::Rc;

use crate::prelude::*;
use client::{use_base_url, use_pausable_request, UseFetchHandle};

use reqwest::{Method, Request};

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

    let base_url = use_base_url();

    let error_clone = error.clone();
    let meta: Option<Rc<Metadata>> = match use_pausable_request(move || {
        let mut url = match base_url {
            Some(m) => m,

            None => {
                error_clone.set(ErrorKind::Unknown);
                return None;
            }
        };

        url.set_path("/metadata.json");

        Some(Request::new(Method::GET, url))
    }) {
        UseFetchHandle::Loading => None,
        UseFetchHandle::Ok(m) => {
            let data: Rc<Metadata> = m.data();

            Some(data)
        }
        UseFetchHandle::Err(_) => {
            error.set(ErrorKind::Server);

            None
        }
    };

    let state = MetadataState { value: meta };

    html! {
        <ContextProvider<MetadataState> context={state}>
            {children}
        </ContextProvider<MetadataState>>
    }
}
