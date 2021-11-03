use std::cell::RefCell;
use std::rc::{Rc, Weak};

use anymap2::any::CloneAny;
use anymap2::Map;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::slice::Slice;
use crate::utils::Id;

pub(crate) type AtomMap = Map<dyn CloneAny>;
type ListenerVec = Rc<RefCell<Vec<Weak<Callback<BounceRootState>>>>>;

#[derive(Properties, Debug, PartialEq)]
pub struct BounceRootProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct SliceListener {
    _listener: Rc<Callback<BounceRootState>>,
}

#[derive(Clone)]
pub(crate) struct BounceRootState {
    id: Id,
    atoms: Rc<RefCell<AtomMap>>,
    listeners: ListenerVec,
}

impl BounceRootState {
    pub(crate) fn dispatch_action<T>(&self, val: T::Action)
    where
        T: Slice + 'static,
    {
        let mut atoms = self.atoms.borrow_mut();
        let prev_val = atoms.remove::<Rc<T>>().unwrap_or_default();
        let next_val = Rc::new(prev_val.reduce(val));

        let should_notify = prev_val != next_val;

        atoms.insert(next_val);

        if should_notify {
            self.notify_listeners();
        }
    }

    pub(crate) fn listen<CB>(&self, callback: CB) -> SliceListener
    where
        CB: Fn(BounceRootState) + 'static,
    {
        let cb = Rc::new(Callback::from(callback));

        self.listeners.borrow_mut().push(Rc::downgrade(&cb));

        SliceListener { _listener: cb }
    }

    pub(crate) fn get<T>(&self) -> Rc<T>
    where
        T: Slice + 'static,
    {
        let mut atoms = self.atoms.borrow_mut();
        if let Some(m) = atoms.get::<Rc<T>>().cloned() {
            m
        } else {
            let val = Rc::new(T::default());
            atoms.insert(val.clone());
            val
        }
    }

    fn notify_listeners_impl(&self) {
        let callables = {
            let mut callbacks_ref = self.listeners.borrow_mut();

            // Any gone weak references are removed when called.
            let (callbacks, callbacks_weak) = callbacks_ref.iter().cloned().fold(
                (Vec::new(), Vec::new()),
                |(mut callbacks, mut callbacks_weak), m| {
                    if let Some(m_strong) = m.clone().upgrade() {
                        callbacks.push(m_strong);
                        callbacks_weak.push(m);
                    }

                    (callbacks, callbacks_weak)
                },
            );

            *callbacks_ref = callbacks_weak;

            callbacks
        };

        for callback in callables {
            callback.emit(self.to_owned())
        }
    }

    pub(crate) fn notify_listeners(&self) {
        let self_ = self.to_owned();
        spawn_local(async move {
            self_.notify_listeners_impl();
        });
    }
}

impl PartialEq for BounceRootState {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

#[function_component(BounceRoot)]
pub fn bounce_root(props: &BounceRootProps) -> Html {
    let children = props.children.clone();

    let root_state = use_state(|| BounceRootState {
        id: Id::new(),
        atoms: Rc::default(),
        listeners: Rc::default(),
    });

    html! {
        <ContextProvider<BounceRootState> context={(*root_state).clone()}>{children}</ContextProvider<BounceRootState>>
    }
}
