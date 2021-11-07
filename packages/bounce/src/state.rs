use std::rc::Rc;

use crate::utils::sealed::Sealed;

pub trait Stateful: Sized {
    type State: State<Self>;
    type Input;
}

pub trait State<T>: Sealed + Default + Clone
where
    T: Stateful,
{
    fn new() -> Self;
    fn get(&mut self) -> Rc<T>;
    fn set(&mut self, val: T::Input) -> bool;
}
