use std::rc::Rc;

use crate::slice::Slice;

pub use fl_www_macros::Atom;

pub trait Atom: PartialEq + Default {}

impl<T> Slice for T
where
    T: Atom,
{
    type Action = T;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}
