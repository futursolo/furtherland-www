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
    let error = use_atom::<ErrorState>();

    match use_query(|| Request::builder().url("/metadata.json").build()).result() {
        Some(Ok(m)) => Some(m.data()),
        Some(Err(_)) => {
            error.set(ErrorKind::Server.into());

            None
        }
        _ => None,
    }
}
