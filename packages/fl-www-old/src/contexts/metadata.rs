use std::ops::Deref;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MetadataState {
    value: &'static Metadata,
}

impl Default for MetadataState {
    fn default() -> Self {
        Self {
            value: Metadata::get(),
        }
    }
}

impl Deref for MetadataState {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub(crate) fn use_metadata() -> MetadataState {
    use_context::<MetadataState>().unwrap()
}

#[function_component(MetaProvider)]
pub(crate) fn meta_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    html! {
        <ContextProvider<MetadataState> context={MetadataState::default()}>
            {children}
        </ContextProvider<MetadataState>>
    }
}
