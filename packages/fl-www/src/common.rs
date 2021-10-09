use crate::prelude::*;

#[derive(Properties, Debug, Clone, PartialEq)]
pub(crate) struct ChildrenProps {
    #[prop_or_default]
    pub children: Children,
}
