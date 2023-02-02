use yew::prelude::*;
use std::rc::Rc;
use rustgie::types::destiny::responses::DestinyProfileResponse;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct ProviderProps {
    pub children: Children,
    pub profile: DestinyProfileResponse,
}

#[function_component(ProfileProvider)]
pub fn provider(props: &ProviderProps) -> Html {
    let profile = use_memo(
        |profile| profile.to_owned(),
        props.profile.to_owned()
    );

    html! {
        <ContextProvider<Rc<DestinyProfileResponse>> context={profile}>
          { for props.children.iter() }
        </ContextProvider<Rc<DestinyProfileResponse>>>
    }
}

