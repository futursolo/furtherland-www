use std::rc::Rc;

use fl_www_api::{Bridge, Link, MetadataQuery};
use stellation_bridge::hooks::UseBridgedQueryHandle;
use yew::prelude::*;
use yew::suspense::SuspensionResult;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct MetadataState {
    value: Option<Rc<Metadata>>,
}

#[hook]
pub(crate) fn use_metadata() -> SuspensionResult<UseBridgedQueryHandle<MetadataQuery, Link>> {
    Bridge::use_query::<MetadataQuery>(().into())
}
