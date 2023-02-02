use yew::prelude::*;
use std::rc::Rc;
use yew_hooks::prelude::*;
use crate::components::wheel::LoadedWheel;
use crate::components::fireteam::FireteamWrapper;
use crate::components::profile::ProfileWrapper;
use js_sys::Math::{floor, random};
use log::info;

use yew::html;
use yew::suspense::{Suspension, SuspensionResult, use_future, UseFutureHandle};
use data::api::manifest::model::Item;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct WheelProps {
    pub players: Vec<String>,
}


#[function_component(Wheel)]
pub fn wheel() -> Html {
    let fallback = html! {
        <div>{"Loading..."}</div>
    };


    html! {
        <div class="app">
            <h1>{"Wheel of Misfortune"}</h1>
            <ProfileWrapper>
                <FireteamWrapper>
                    <Suspense {fallback}>
                        <LoadedWheel />
                    </Suspense>
                </FireteamWrapper>
            </ProfileWrapper>
        </div>
    }
}
