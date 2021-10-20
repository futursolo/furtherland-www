use std::rc::Rc;
use std::str::FromStr;
use web_sys::window;

use yew::prelude::*;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::handle::UseFetchHandle;
use crate::provider::ClientState;
use crate::request::Request;
use crate::response::Response;
use futures::future::{ready, TryFutureExt};
use futures::lock::Mutex;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub fn use_client() -> Option<Rc<Client>> {
    use_context::<ClientState>().map(|m| m.inner)
}

pub async fn fetch<T, E>(client: &Client, req: Request) -> Result<Response<T>, E>
where
    T: FromStr<Err = E> + Clone + 'static,
    E: std::error::Error + 'static,
{
    let window = window().ok_or_else(|| Rc::new(Error::Web(None)))?;
    let req = req.to_fetch_request::<E>(client)?;
    let resp = JsFuture::from(window.fetch_with_request(&req))
        .await
        .and_then(|m| m.dyn_into::<web_sys::Response>())
        .map_err(|e| Error::Web(Some(e)))?;

    if resp.status() >= 400 {
        return Err(Rc::new(Error::Response(resp)));
    }

    let headers = resp.headers().to_owned();
    let data_s = ready(resp.text().map(JsFuture::from))
        .try_flatten()
        .await
        .map_err(|e| Rc::new(Error::Fetch(e)))
        .and_then(|m| m.as_string().ok_or_else(|| Rc::new(Error::Web(None))))?;

    let data = data_s.parse::<T>().map_err(|e| Rc::new(Error::Parse(e)))?;

    Ok(Response {
        data: Rc::new(data),
        headers,
    })
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

                let result = fetch(&client, req).await;

                state_clone.set(UseFetchHandle {
                    result: Some(result),
                });
            }
        });

        || {}
    });

    (*state).clone()
}
