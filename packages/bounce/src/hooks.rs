use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::atom::Atom;
use crate::provider::BounceRootState;
use crate::slice::Slice;

pub struct UseSliceHandle<T>
where
    T: Slice,
{
    inner: Rc<T>,
    root: BounceRootState,
}

impl<T> UseSliceHandle<T>
where
    T: Slice + 'static,
{
    pub fn dispatch(&self, action: T::Action) {
        self.root.dispatch_action::<T>(action);
    }
}

impl<T> Deref for UseSliceHandle<T>
where
    T: Slice,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseSliceHandle<T>
where
    T: Slice,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            root: self.root.clone(),
        }
    }
}

impl<T> fmt::Debug for UseSliceHandle<T>
where
    T: Slice + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseSliceHandle")
            .field("inner", &self.inner)
            .finish()
    }
}

pub fn use_slice<T>() -> UseSliceHandle<T>
where
    T: Slice + 'static,
{
    let root = use_context::<BounceRootState>().expect_throw("No bounce root found.");

    let root_clone = root.clone();
    let val = use_state(move || -> RefCell<Rc<T>> { root_clone.get::<T>().into() });

    let val_clone = val.clone();
    let root_clone = root.clone();
    use_state(move || {
        root_clone.listen(move |root| {
            let next_val = root.get::<T>();
            let prev_val = val_clone.borrow().clone();

            if prev_val != next_val {
                val_clone.set(RefCell::new(next_val));
            }
        })
    });

    let val = (*(*val).borrow()).clone();

    UseSliceHandle { inner: val, root }
}

pub struct UseAtomHandle<T>
where
    T: Atom,
{
    inner: UseSliceHandle<T>,
}

impl<T> UseAtomHandle<T>
where
    T: Atom + 'static,
{
    pub fn set(&self, val: T) {
        self.inner.dispatch(val)
    }
}

impl<T> Deref for UseAtomHandle<T>
where
    T: Atom,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseAtomHandle<T>
where
    T: Atom,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> fmt::Debug for UseAtomHandle<T>
where
    T: Atom + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseAtomHandle")
            .field("inner", &self.inner)
            .finish()
    }
}

pub fn use_atom<T>() -> UseAtomHandle<T>
where
    T: Atom + 'static,
{
    let inner = use_slice::<T>();

    UseAtomHandle { inner }
}
