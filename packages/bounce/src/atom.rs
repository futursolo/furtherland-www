use crate::slice::Slice;

pub trait Atom: PartialEq + Default + Clone {}

impl<T> Slice for T
where
    T: Atom,
{
    type Action = T;

    fn reduce(&self, action: Self::Action) -> Self {
        action
    }
}
