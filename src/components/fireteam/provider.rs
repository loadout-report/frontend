use yew::prelude::*;
use std::rc::Rc;
use rustgie::types::destiny::responses::DestinyProfileResponse;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct ProviderProps {
    pub children: Children,
    pub fireteam: Vec<DestinyProfileResponse>,
}

#[function_component(FireteamProvider)]
pub fn provider(props: &ProviderProps) -> Html {
    let fireteam = use_memo(
        |fireteam| fireteam.to_owned(),
        props.fireteam.to_owned()
    );

    html! {
        <ContextProvider<Rc<Vec<DestinyProfileResponse>>> context={fireteam}>
          { for props.children.iter() }
        </ContextProvider<Rc<Vec<DestinyProfileResponse>>>>
    }
}

