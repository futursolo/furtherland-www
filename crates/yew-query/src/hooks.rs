use std::rc::Rc;
use std::str::FromStr;

use futures::lock::Mutex;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::client::Client;
use crate::error::Error;
use crate::handle::UseFetchHandle;
use crate::provider::ClientState;
use crate::request::Request;

pub fn use_client() -> Option<Rc<Client>> {
    use_context::<ClientState>().map(|m| m.inner)
}

pub fn use_query<T, F, E>(req_fn: F) -> UseFetchHandle<T, E>
where
    T: FromStr<Err = E> + Clone + 'static,
    F: FnOnce() -> Request + 'static,
    E: std::error::Error + 'static,
{
    use_pausable_query(move || Some(req_fn()))
}

pub fn use_pausable_query<T, F, E>(req_fn: F) -> UseFetchHandle<T, E>
where
    T: FromStr<Err = E> + Clone + 'static,
    F: FnOnce() -> Option<Request> + 'static,
    E: std::error::Error + 'static,
{
    let client = use_client().unwrap_or_default();

    let state = use_state(|| UseFetchHandle { result: None });
    let dispatched = use_state(|| -> Mutex<Option<Request>> { Mutex::new(None) });

    let state_clone = state.clone();
    use_effect(move || {
        spawn_local(async move {
            let mut dispatched = match dispatched.try_lock() {
                Some(m) => m,
                None => return,
            };

            if let Some(req) = req_fn() {
                if (*dispatched).as_ref() == Some(&req) {
                    return;
                }

                *dispatched = Some(req.clone());
                state_clone.set(UseFetchHandle { result: None });

                let result = client
                    .fetch(req)
                    .await
                    .map_err(|e| Rc::new(Error::from(e).cast_parse_err::<E>()));

                state_clone.set(UseFetchHandle {
                    result: Some(result.and_then(|m| m.into_response::<T, E>())),
                });
            }
        });

        || {}
    });

    (*state).clone()
}
