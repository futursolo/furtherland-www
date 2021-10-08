use gloo::events::EventListener;
use web_sys::{Event, EventTarget};

use crate::prelude::*;
use utils::Id;

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
