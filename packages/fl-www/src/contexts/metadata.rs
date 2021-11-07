use std::rc::Rc;

use bounce::prelude::*;
use yew_query::{use_query, Request};

use crate::prelude::*;

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
    let set_error = use_set_bounce_value::<ErrorState>();

    match use_query(|| Request::builder().url("/metadata.json").build()).result() {
        Some(Ok(m)) => Some(m.data()),
        Some(Err(_)) => {
            set_error(ErrorKind::Server.into());

            None
        }
        _ => None,
    }
}
