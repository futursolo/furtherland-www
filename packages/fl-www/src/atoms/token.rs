use bounce::prelude::*;

#[derive(Atom, Default, PartialEq)]
pub(crate) struct TokenState {
    pub inner: Option<String>,
}
