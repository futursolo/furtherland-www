use gloo::events::EventListener;
use web_sys::{Event, EventTarget};
use yew::prelude::*;

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
