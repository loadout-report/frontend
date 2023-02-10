use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;
use anyhow::bail;
use rustgie::types::destiny::responses::DestinyProfileResponse;
use rustgie::types::user::{ExactSearchRequest, UserInfoCard};
use tracing::{info, warn};

use yew::prelude::*;
use yew::suspense::{use_future, UseFutureHandle};
use crate::client::Client;
use crate::components::wheel::RollOption;
use super::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct WrapperProps {
    pub children: Children,
}

#[function_component(ProfileWrapper)]
pub fn wrapper(props: &WrapperProps) -> Html {
    let fallback = html!("loading profile");
    let username = use_state(|| None);

    let onprofile = {
        let username_handle = username.clone();
        Callback::from(move |search_request| {
            let username_handle = username_handle.clone();
            info!("user entered: {:?}", search_request);
            username_handle.set(Some(search_request));
        })
    };

    html! {
        <>
          <ProfileSelector {onprofile} />
          if username.is_some() {
              <Suspense {fallback}>
                <AsyncProfileProvider search_request={username.as_ref().unwrap().clone()}>
                  { for props.children.iter() }
                </AsyncProfileProvider>
              </Suspense>
          }
        </>
    }
}

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct AsyncProfileProviderProperties {
    pub children: Children,
    pub search_request: ExactSearchRequest,
}

#[function_component(AsyncProfileProvider)]
pub fn async_profile_provider(properties: &AsyncProfileProviderProperties) -> HtmlResult {
    let client: Rc<Client> = use_context::<Rc<Client>>().expect("Client not initialised");
    let profile: UseFutureHandle<Result<DestinyProfileResponse, anyhow::Error>> = {
        info!("creating async profile provider handle");
        let search_request = properties.search_request.clone();
        let client = client.clone();
        use_future(|| async move {
            let client = client.clone();
            let search_request = search_request.clone();
            info!("searching for profile: {:?}", search_request);
            let user_info: Option<Vec<UserInfoCard>> = client.search(search_request).await.inspect_err(|err| warn!("error searching for profile: {:?}", err)).ok();
            if let Some(info) = user_info.and_then(|info| {
                let first = info.first();
                first.cloned()
            }) {
                if let Ok(profile) = client.get_profile(info.membership_type as i32, info.membership_id).await.inspect_err(|err| warn!("error getting profile: {:?}", err)) {
                    return Ok(profile);
                }
            }
            bail!("no profile found")
        })?
    };

    let template = match *profile {
        Ok(ref res) => html! {
            <ProfileProvider profile={res.clone()}>
                { for properties.children.iter() }
            </ProfileProvider>
        },
        Err(ref failure) => failure.to_string().into()
    };

    Ok(html! {
        <>
            {template}
        </>
    })
}
