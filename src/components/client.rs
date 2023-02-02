use yew::prelude::*;
use std::rc::Rc;
use crate::client::Client;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct ProviderProps {
    pub children: Children,
    pub client: Client,
}

#[function_component(ClientProvider)]
pub fn provider(props: &ProviderProps) -> Html {
    let client = use_memo(|client| client.to_owned(), props.client.to_owned());

    html! {
        <ContextProvider<Rc<Client>> context={client}>
          { for props.children.iter() }
        </ContextProvider<Rc<Client>>>
    }
}

