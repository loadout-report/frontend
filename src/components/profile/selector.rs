use log::info;
use rustgie::types::user::ExactSearchRequest;
use serde_derive::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;


#[derive(Clone, PartialEq, Properties)]
pub struct SelectorProps {
    pub onprofile: Callback<ExactSearchRequest>,
}

#[function_component(ProfileSelector)]
pub fn player_selector(props: &SelectorProps) -> Html {


    let onprofile = {
        let onprofile = props.onprofile.clone();
        move |player| onprofile.emit(player)
    };

    let onkeypress = {

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let target = e.target_unchecked_into::<HtmlInputElement>();
                let value: String = target.value();
                onprofile(to_exact_search_request(value.as_str()));
            }
        }
    };

    html! {
        <div>
            <h2>{"Profile Selector"}</h2>
            <input type="text" {onkeypress} />
            // <button {onclick} type="button">{"Search"}</button>
        </div>
    }
}

fn to_exact_search_request(s: &str) -> ExactSearchRequest {
        let mut split = s.split('#');
        let name = split.next().unwrap_or_default().to_string();
        let code = split.next().unwrap_or_default().to_string();
        let code = code.parse().unwrap_or_default();
        ExactSearchRequest { display_name: Some(name.to_string()), display_name_code: code }
}
