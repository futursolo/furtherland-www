// use std::rc::Rc;

// use crate::state::State;
// use crate::{Atom, Slice};

// pub struct SelectorGetHandle<T>
// where
//     T: Selector + 'static,
// {
//     inner: Rc<T>,
// }

// impl<S> SelectorGetHandle<S>
// where
//     S: Selector + 'static,
// {
//     pub fn get<T>(&self) -> Rc<T>
//     where
//         T: Slice + 'static,
//     {
//         todo!()
//     }
// }

// pub struct SelectorHandle<T>
// where
//     T: Selector + 'static,
// {
//     inner: Rc<T>,
// }

// impl<S> SelectorHandle<S>
// where
//     S: Selector + 'static,
// {
//     pub fn get<T>(&self) -> Rc<T>
//     where
//         T: Slice + 'static,
//     {
//         todo!()
//     }

//     pub fn set<T>(&self, val: T)
//     where
//         T: Atom + 'static,
//     {
//         todo!()
//     }
// }

// pub trait Selector: Sized {
//     type Input;

//     fn get(handle: SelectorGetHandle<Self>) -> Rc<Self>;
//     fn set(val: Self::Input, handle: SelectorHandle<Self>) {
//         unimplemented!("You need to implement reduce before using it.")
//     }
// }
