use crate::prelude::*;
use std::rc::Rc;

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

#[derive(Debug)]
pub enum UseFetchHandle {
    Ok(Rc<reqwest::Response>),
    Loading,
    Err(Rc<reqwest::Error>),
}

impl UseFetchHandle {
    pub fn into_result(
        self,
    ) -> Option<std::result::Result<Rc<reqwest::Response>, Rc<reqwest::Error>>> {
        match self {
            Self::Ok(m) => Some(Ok(m)),
            Self::Loading => None,
            Self::Err(e) => Some(Err(e)),
        }
    }
}

impl Clone for UseFetchHandle {
    fn clone(&self) -> Self {
        match self {
            Self::Ok(m) => Self::Ok(m.clone()),
            Self::Loading => Self::Loading,
            Self::Err(e) => Self::Err(e.clone()),
        }
    }
}

pub fn use_request(_req: reqwest::Request) -> UseFetchHandle {
    todo!();
}
