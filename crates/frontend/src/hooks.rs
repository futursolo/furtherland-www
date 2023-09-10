use std::rc::Rc;

use gloo::events::EventListener;
use utils::{get_viewport_height, Id};
use web_sys::{Event, EventTarget};
use yew::prelude::*;

use crate::prelude::*;

#[hook]
pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(&Event) + 'static,
{
    #[derive(Clone)]
    struct EventDependents {
        target: EventTarget,
        event_type: Cow<'static, str>,
        callback: Rc<dyn Fn(&Event)>,
    }

    #[allow(clippy::vtable_address_comparisons)]
    impl PartialEq for EventDependents {
        fn eq(&self, rhs: &Self) -> bool {
            self.target == rhs.target
                && self.event_type == rhs.event_type
                && Rc::ptr_eq(&self.callback, &rhs.callback)
        }
    }

    let deps = EventDependents {
        target: target.clone(),
        event_type: event_type.into(),
        callback: Rc::new(callback) as Rc<dyn Fn(&Event)>,
    };

    use_effect_with_deps(
        |deps| {
            let EventDependents {
                target,
                event_type,
                callback,
            } = deps.clone();

            let listener = EventListener::new(&target, event_type, move |e| {
                callback(e);
            });

            move || {
                drop(listener);
            }
        },
        deps,
    );
}

#[hook]
pub fn use_render_event<E>(target: &EventTarget, event_type: E)
where
    E: Into<Cow<'static, str>>,
{
    let ctr = use_state(Id::new);

    use_event(target, event_type, move |_e| {
        ctr.set(Id::new());
    });
}

#[hook]
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
