use std::rc::Rc;

use yew::prelude::*;

use crate::client::Client;

#[derive(Debug, Clone)]
pub struct ClientState {
    // We use Rc here for PartialEq
    pub(crate) inner: Rc<Client>,
}

impl PartialEq for ClientState {
    fn eq(&self, rhs: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &rhs.inner)
    }
}

#[derive(Properties, Debug, Clone)]
pub struct ClientProviderProps {
    pub client: Rc<Client>,
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
