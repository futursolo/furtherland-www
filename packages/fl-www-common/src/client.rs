use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use reqwest::header::HeaderMap;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ClientState {
    inner: Rc<reqwest::Client>,
}

impl PartialEq for ClientState {
    fn eq(&self, rhs: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &rhs.inner)
    }
}

#[derive(Properties, Debug, Clone)]
pub struct ClientProviderProps {
    pub client: Rc<reqwest::Client>,
    #[prop_or_default]
    pub children: Children,
}

impl PartialEq for ClientProviderProps {
    fn eq(&self, rhs: &Self) -> bool {
        Rc::ptr_eq(&self.client, &rhs.client) && self.children == rhs.children
    }
}

#[function_component(ClientProvider)]
pub fn client_provider(props: &ClientProviderProps) -> Html {
    let children = props.children.clone();
    let state = ClientState {
        inner: props.client.clone(),
    };

    html! {<ContextProvider<ClientState> context={state}>{children}</ContextProvider<ClientState>>}
}

pub fn use_client() -> reqwest::Client {
    use_context::<ClientState>()
        .map(|m| (*m.inner).clone())
        .unwrap_or_else(reqwest::Client::new)
}

#[derive(Debug, Clone)]
pub struct ClientResponse<T>
where
    T: Clone + 'static,
{
    // inner: Rc<reqwest::Response>,
    data: Rc<T>,
    headers: Rc<HeaderMap>,
}

impl<T> ClientResponse<T>
where
    T: Clone + 'static,
{
    pub fn data(&self) -> Rc<T> {
        self.data.clone()
    }

    pub fn headers(&self) -> Rc<HeaderMap> {
        self.headers.clone()
    }
}

#[derive(Error, Debug)]
pub enum ClientError<E>
where
    E: std::error::Error + 'static,
{
    #[error("Failed to parse")]
    Parse(#[source] E),

    #[error("Failed to communicate with server")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug)]
pub enum UseFetchHandle<T, E>
where
    T: Clone + 'static,
    E: std::error::Error + 'static,
{
    Ok(ClientResponse<T>),
    Loading,
    Err(Rc<ClientError<E>>),
}

impl<T, E> UseFetchHandle<T, E>
where
    T: Clone + 'static,
    E: std::error::Error + 'static,
{
    pub fn into_result(self) -> Option<std::result::Result<ClientResponse<T>, Rc<ClientError<E>>>> {
        match self {
            Self::Ok(m) => Some(Ok(m)),
            Self::Loading => None,
            Self::Err(e) => Some(Err(e)),
        }
    }
}

impl<T, E> Clone for UseFetchHandle<T, E>
where
    T: Clone + 'static,
    E: std::error::Error + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::Ok(m) => Self::Ok(m.clone()),
            Self::Loading => Self::Loading,
            Self::Err(e) => Self::Err(e.clone()),
        }
    }
}

pub fn use_request<T, F, E>(req_fn: F) -> UseFetchHandle<T, E>
where
    T: FromStr<Err = E> + Clone + 'static,
    F: FnOnce() -> reqwest::Request + 'static,
    E: std::error::Error + 'static,
{
    use_pausable_request(move || Some(req_fn()))
}

pub fn use_pausable_request<T, F, E>(req_fn: F) -> UseFetchHandle<T, E>
where
    T: FromStr<Err = E> + Clone + 'static,
    F: FnOnce() -> Option<reqwest::Request> + 'static,
    E: std::error::Error + 'static,
{
    let client = use_client();

    let state = use_state(|| UseFetchHandle::Loading);
    let dispatched = use_state(|| RefCell::new(false));

    let state_clone = state.clone();
    use_effect(move || {
        let mut dispatched_ = dispatched.borrow_mut();

        if !*dispatched_ {
            if let Some(req) = req_fn() {
                *dispatched_ = true;

                spawn_local(async move {
                    let resp_result = client.execute(req).await;

                    let resp = match resp_result.and_then(|m| m.error_for_status()) {
                        Ok(m) => m,
                        Err(e) => {
                            state_clone.set(UseFetchHandle::Err(Rc::new(ClientError::from(e))));
                            return;
                        }
                    };
                    let headers = resp.headers().to_owned();

                    let data_s = match resp.text().await {
                        Ok(m) => m,
                        Err(e) => {
                            state_clone.set(UseFetchHandle::Err(Rc::new(ClientError::from(e))));
                            return;
                        }
                    };

                    let data = match data_s.parse::<T>() {
                        Ok(m) => m,
                        Err(e) => {
                            state_clone.set(UseFetchHandle::Err(Rc::new(ClientError::Parse(e))));

                            return;
                        }
                    };

                    let resp = ClientResponse {
                        data: Rc::new(data),
                        headers: Rc::new(headers),
                    };

                    state_clone.set(UseFetchHandle::Ok(resp));
                });
            };
        }

        || {}
    });

    (*state).clone()
}
