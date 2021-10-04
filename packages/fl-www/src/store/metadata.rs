use crate::prelude::*;

#[derive(Debug, Clone)]
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

impl MetadataState {
    pub fn current(&self) -> &Metadata {
        self.value
    }
}
