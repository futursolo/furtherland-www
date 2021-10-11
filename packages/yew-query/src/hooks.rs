use std::rc::Rc;
use std::str::FromStr;
use web_sys::window;

use yew::prelude::*;

use crate::client::Client;
use crate::error::Error;
use crate::handle::UseFetchHandle;
use crate::provider::ClientState;
use crate::request::Request;
use crate::response::Response;
use futures::future;
use futures::future::{ready, FutureExt, TryFutureExt};
use futures::lock::Mutex;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub fn use_client() -> Option<Rc<Client>> {
    use_context::<ClientState>().map(|m| m.inner)
}

// pub fn use_base_url() -> Option<Url> {
//     let default_url = use_state(|| {
//         window()
//             .location()
//             .href()
//             .ok()
//             .and_then(|m| Url::parse(&m).ok())
//             .map(|mut m| {
//                 m.set_path("/");
//                 m
//             })
//     });

//     use_context::<ClientState>()
//         .and_then(|m| m.base_url)
//         .or_else(|| (*default_url).clone())
// }

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
            let mut dispatched = dispatched.lock().await;

            if let Some(req) = req_fn() {
                if (*dispatched).as_ref() == Some(&req) {
                    return;
                }

                *dispatched = Some(req.clone());
                state_clone.set(UseFetchHandle { result: None });

                let resp = match ready(
                    window()
                        .ok_or(Error::Web(None))
                        .and_then(|m| req.to_fetch_request(&client).map(|req| (m, req)))
                        .map(|(window, req)| JsFuture::from(window.fetch_with_request(&req)))
                        .map(|m| {
                            m.map(|m| {
                                m.and_then(|m| m.dyn_into::<web_sys::Response>())
                                    .map_err(|e| Error::Web(Some(e)))
                            })
                        }),
                )
                .try_flatten()
                .and_then(|m| {
                    if m.status() >= 400 {
                        future::err(Error::Response(m))
                    } else {
                        future::ok(m)
                    }
                })
                .await
                {
                    Ok(m) => m,
                    Err(e) => {
                        state_clone.set(UseFetchHandle {
                            result: Some(Err(Rc::new(e.cast_parse_err::<E>()))),
                        });
                        return;
                    }
                };

                let headers = resp.headers().to_owned();

                let data = match future::ready(resp.text().map(JsFuture::from))
                    .try_flatten()
                    .map_err(|e| Rc::new(Error::Fetch(e)))
                    .and_then(|m| ready(m.as_string().ok_or_else(|| Rc::new(Error::Web(None)))))
                    .and_then(|m| ready(m.parse::<T>().map_err(|e| Rc::new(Error::Parse(e)))))
                    .await
                {
                    Ok(m) => m,
                    Err(e) => {
                        state_clone.set(UseFetchHandle {
                            result: Some(Err(e)),
                        });
                        return;
                    }
                };

                let resp = Response {
                    data: Rc::new(data),
                    headers,
                };

                state_clone.set(UseFetchHandle {
                    result: Some(Ok(resp)),
                });
            }
        });

        || {}
    });

    (*state).clone()
}
