use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;
use anyhow::bail;
use rustgie::types::destiny::responses::DestinyProfileResponse;
use rustgie::types::user::{ExactSearchRequest, UserInfoCard};
use tracing::{info, warn};
use web_sys::async;

use yew::prelude::*;
use yew::props;
use yew::suspense::{Suspension, SuspensionResult, use_future, use_future_with_deps, UseFutureHandle};
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
    let username: UseStateHandle<Option<ExactSearchRequest>> = use_state(|| None);

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
                <AsyncProfileProvider display_name={username.as_ref().unwrap().clone().display_name.unwrap()} code={username.as_ref().unwrap().display_name_code}>
                  { for props.children.iter() }
                </AsyncProfileProvider>
              </Suspense>
          }
        </>
    }
}

#[hook]
fn use_profile(display_name: &str, code: i16) -> SuspensionResult<DestinyProfileResponse> {

}

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct AsyncProfileProviderProperties {
    pub children: Children,
    pub display_name: AttrValue,
    pub code: i16,
}

#[function_component(AsyncProfileProvider)]
pub fn async_profile_provider(properties: &AsyncProfileProviderProperties) -> HtmlResult {
    let client: Rc<Client> = use_context::<Rc<Client>>().expect("Client not initialised");
    let display_name = properties.display_name.clone();
    let display_name = display_name.to_string();
    let profile: UseFutureHandle<Result<DestinyProfileResponse, anyhow::Error>> = {
        info!("creating async profile provider handle");
        let client = client.clone();
        use_future_with_deps(|arguments| async move {
            let client = client.clone();
            let arguments = arguments.as_ref().clone();
            info!("searching for profile: {:?}", arguments.0.clone());
            let user_info: Option<Vec<UserInfoCard>> = client
                .search(ExactSearchRequest {
                    display_name: Some(arguments.0.to_string()),
                    display_name_code: arguments.1
                })
                .await.inspect_err(|err| warn!("error searching for profile: {:?}", err)).ok();
            if let Some(info) = user_info.and_then(|info| {
                let first = info.first();
                first.cloned()
            }) {
                if let Ok(profile) = client.get_profile(info.membership_type as i32, info.membership_id).await.inspect_err(|err| warn!("error getting profile: {:?}", err)) {
                    return Ok(profile);
                }
            }
            bail!("no profile found")
        }, (display_name, properties.code))?
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
