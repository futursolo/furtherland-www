use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use gloo::events::EventListener;
use web_sys::{Event, EventTarget};
use yew::functional::use_hook;

use crate::prelude::*;
use utils::{get_viewport_height, Id};

pub fn use_event<E, F>(target: &EventTarget, event_type: E, mut callback: F)
where
    E: Into<Cow<'static, str>>,
    F: FnMut(&Event) + 'static,
{
    use_state(move || {
        EventListener::new(target, event_type, move |e| {
            callback(e);
        })
    });
}

pub fn use_render_event<E>(target: &EventTarget, event_type: E)
where
    E: Into<Cow<'static, str>>,
{
    let ctr = use_state(Id::new);

    use_event(target, event_type, move |_e| {
        ctr.set(Id::new());
    });
}

pub fn use_viewport_height() -> u64 {
    let vh = use_state_eq(get_viewport_height);

    let vh_clone = vh.clone();
    use_event(&window(), "resize", move |_| {
        vh_clone.set(get_viewport_height())
    });

    let vh_clone = vh.clone();
    use_event(&window(), "orientationchange", move |_| {
        vh_clone.set(get_viewport_height())
    });

    let vh_clone = vh.clone();
    use_event(&window(), "scroll", move |_| {
        vh_clone.set(get_viewport_height())
    });

    *vh
}

type DispatchFn<T> = Rc<dyn Fn(<T as Reduce>::Action)>;

pub trait Reduce {
    type Action;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self>;
}

struct UseReducer<T>
where
    T: Reduce + 'static,
{
    current_state: Rc<T>,

    // To be replaced with OnceCell once it becomes available in std.
    dispatch: RefCell<Option<DispatchFn<T>>>,
}

pub struct UseReducerHandle<T>
where
    T: Reduce,
{
    value: Rc<T>,
    dispatch: DispatchFn<T>,
}

impl<T> Deref for UseReducerHandle<T>
where
    T: Reduce,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

impl<T> Clone for UseReducerHandle<T>
where
    T: Reduce,
{
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            dispatch: Rc::clone(&self.dispatch),
        }
    }
}

impl<T> fmt::Debug for UseReducerHandle<T>
where
    T: Reduce + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseReducerHandle")
            .field("value", &format!("{:?}", self.value))
            .finish()
    }
}

impl<T> PartialEq for UseReducerHandle<T>
where
    T: Reduce + PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

impl<T> UseReducerHandle<T>
where
    T: Reduce,
{
    /// Calls the dispatch with the given value
    pub fn dispatch(&self, value: T::Action) {
        (self.dispatch)(value)
    }
}

fn use_reducer_base<T, F, R>(initial_fn: F, should_render_fn: R) -> UseReducerHandle<T>
where
    T: Reduce + 'static,
    F: FnOnce() -> T,
    R: (Fn(&T, &T) -> bool) + 'static,
{
    use_hook(
        move || UseReducer {
            current_state: Rc::new(initial_fn()),
            dispatch: RefCell::default(),
        },
        |s, updater| {
            let mut dispatch_ref = s.dispatch.borrow_mut();

            // Create dispatch once.
            let dispatch = match *dispatch_ref {
                Some(ref m) => (*m).to_owned(),
                None => {
                    let should_render_fn = Rc::new(should_render_fn);

                    let dispatch: Rc<dyn Fn(T::Action)> = Rc::new(move |action: T::Action| {
                        let should_render_fn = should_render_fn.clone();

                        updater.callback(move |state: &mut UseReducer<T>| {
                            let next_state = state.current_state.clone().reduce(action);
                            let should_render = should_render_fn(&next_state, &state.current_state);
                            state.current_state = next_state;

                            should_render
                        });
                    });

                    *dispatch_ref = Some(dispatch.clone());

                    dispatch
                }
            };

            UseReducerHandle {
                value: Rc::clone(&s.current_state),
                dispatch,
            }
        },
        |_| {},
    )
}

pub fn use_reducer<T, F>(initial_fn: F) -> UseReducerHandle<T>
where
    T: Reduce + 'static,
    F: FnOnce() -> T,
{
    use_reducer_base(initial_fn, |_, _| true)
}

pub fn use_reducer_eq<T, F>(initial_fn: F) -> UseReducerHandle<T>
where
    T: Reduce + PartialEq + 'static,
    F: FnOnce() -> T,
{
    use_reducer_base(initial_fn, T::ne)
}

pub struct UseStateReducer<T> {
    value: Rc<T>,
}

impl<T> Reduce for UseStateReducer<T> {
    type Action = T;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(Self {
            value: action.into(),
        })
    }
}

impl<T> PartialEq for UseStateReducer<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

pub struct UseStateHandle<T> {
    inner: UseReducerHandle<UseStateReducer<T>>,
}

impl<T> Deref for UseStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner).value
    }
}

impl<T> Clone for UseStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> fmt::Debug for UseStateHandle<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseStateHandle")
            .field("value", &format!("{:?}", (*self.inner).value))
            .finish()
    }
}

impl<T> PartialEq for UseStateHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
impl<T> UseStateHandle<T> {
    pub fn set(&self, value: T) {
        self.inner.dispatch(value)
    }
}

pub fn use_state<T, F>(init_fn: F) -> UseStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let handle = use_reducer(move || UseStateReducer {
        value: Rc::new(init_fn()),
    });

    UseStateHandle { inner: handle }
}

pub fn use_state_eq<T, F>(init_fn: F) -> UseStateHandle<T>
where
    T: PartialEq + 'static,
    F: FnOnce() -> T,
{
    let handle = use_reducer_eq(move || UseStateReducer {
        value: Rc::new(init_fn()),
    });

    UseStateHandle { inner: handle }
}
