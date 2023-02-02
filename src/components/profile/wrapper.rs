use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;
use log::{info, warn};
use rustgie::types::user::UserInfoCard;

use yew::prelude::*;
use crate::client::Client;
use super::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct WrapperProps {
    pub children: Children,
}

#[function_component(ProfileWrapper)]
pub fn wrapper(props: &WrapperProps) -> Html {

    let profile = use_state(|| None);
    let client: Rc<Client> = use_context::<Rc<Client>>().expect("Client not initialised");

    let onprofile = {
        let client = client.clone();
        let profile_handle = profile.clone();
        Callback::from(move |search_request| {
            let client = client.clone();
            let profile_handle = profile_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                info!("searching for profile: {:?}", search_request);
                let user_info: Option<Vec<UserInfoCard>> = client.search(search_request).await.inspect_err(|err| warn!("error searching for profile: {:?}", err)).ok();
                if let Some(info) = user_info.and_then(|info| {
                    let first = info.first();
                    first.cloned()
                }) {
                    if let Ok(profile) = client.get_profile(info.membership_type as i32, info.membership_id).await.inspect_err(|err| warn!("error getting profile: {:?}", err)) {
                        profile_handle.set(Some(profile))
                    }
                }
            });
        })
    };

    html! {
        <>
          <ProfileSelector {onprofile} />
          if let Some(profile) = profile.deref() {
              <ProfileProvider profile={profile.to_owned()}>
                { for props.children.iter() }
              </ProfileProvider>
          }
        </>
    }
}
