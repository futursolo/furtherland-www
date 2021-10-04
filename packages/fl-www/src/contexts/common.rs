use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct ContextProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,
    pub children: Children,
}

impl_dispatch_mut!(ContextProps);
