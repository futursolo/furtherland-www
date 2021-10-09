use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use gloo::events::EventListener;
use web_sys::{Event, EventTarget};

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
    let vh = use_equal_state(get_viewport_height);

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

    *vh.borrow()
}

#[derive(Debug, PartialEq)]
pub struct UseEqualStateHandle<T>
where
    T: PartialEq + fmt::Debug,
{
    refresh: UseStateHandle<()>,
    inner: Rc<RefCell<Rc<T>>>,
}

impl<T> UseEqualStateHandle<T>
where
    T: PartialEq + fmt::Debug,
{
    pub fn set(&self, value: T) {
        let mut current_value = self.inner.borrow_mut();

        if **current_value != value {
            *current_value = Rc::new(value);

            let refresh = self.refresh.clone();
            spawn_local(async move {
                refresh.set(());
            });
        }
    }

    pub fn borrow(&self) -> Rc<T> {
        self.inner.borrow().clone()
    }
}

impl<T> Clone for UseEqualStateHandle<T>
where
    T: PartialEq + fmt::Debug,
{
    fn clone(&self) -> Self {
        Self {
            refresh: self.refresh.clone(),
            inner: self.inner.clone(),
        }
    }
}

pub fn use_equal_state<T: PartialEq + fmt::Debug + 'static, F: FnOnce() -> T>(
    initial_state_fn: F,
) -> UseEqualStateHandle<T> {
    UseEqualStateHandle {
        refresh: use_state(|| ()),
        inner: (*use_state(move || Rc::new(RefCell::new(Rc::new(initial_state_fn()))))).clone(),
    }
}

// impl<T> Deref for UseEqualStateHandle<T>
// where
//     T: PartialEq + fmt::Debug,
// {
//     type Target = RefCell<T>;

//     fn deref(&self) -> &Self::Target {
//         &*self.inner
//     }
// }

// pub fn use_memo_with_deps<T, F, D>(value_fn: F, deps: D) -> Rc<T>
// where
//     F: Fn(&D) -> T,
//     D: PartialEq + 'static,
//     T: 'static,
// {
//     let generated = use_state(|| -> Option<Rc<T>> { None });
//     let memorised_deps = use_state(|| -> Option<D> { None });

//     if memorised_deps.as_ref() != Some(&deps) {
//         let new_val = Rc::new(value_fn(&deps));

//         generated.set(Some(new_val));
//         memorised_deps.set(Some(deps));
//     }

//     (*generated).to_owned().unwrap()
// }
