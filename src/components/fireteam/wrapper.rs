use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;
use log::{info, warn};
use rustgie::types::destiny::responses::DestinyProfileResponse;
use yew::prelude::*;
use yew::suspense::{use_future, UseFutureHandle};
use crate::client::Client;
use super::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct WrapperProps {
    pub children: Children,
}

#[function_component(FireteamWrapper)]
pub fn wrapper(props: &WrapperProps) -> Html {
    let fallback = html!("loading fireteam");
    let profile: Rc<DestinyProfileResponse> = use_context::<Rc<DestinyProfileResponse>>().expect("Profile not initialised");

    html! {
        <Suspense {fallback}>
            <AsyncFireteamProvider profile={profile.as_ref().clone()}>
                {props.children.clone()}
            </AsyncFireteamProvider>
        </Suspense>
    }
}

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct AsyncFireteamProviderProperties {
    pub children: Children,
    pub profile: DestinyProfileResponse,
}

#[function_component(AsyncFireteamProvider)]
pub fn async_fireteam_provider(properties: &AsyncFireteamProviderProperties) -> HtmlResult {
    let client: Rc<Client> = use_context::<Rc<Client>>().expect("Client not initialised");

    let fireteam: UseFutureHandle<Result<Vec<DestinyProfileResponse>, anyhow::Error>> = {
        let profile = properties.profile.clone();
        let client = client.clone();
        use_future(|| async move {
            let client = client.clone();
            let profile = profile.clone();
            info!("Fetching fireteam");
            let members = profile.clone().profile_transitory_data
                .and_then(|x| x.data)
                .and_then(|x| x.party_members);
            let members = match members {
                Some(m) => m,
                None => {
                    warn!("No party members found. User may not be online.");
                    return Ok(vec![profile.clone()]);
                }
            };

            let membership_id = profile.profile
                .as_ref().to_owned()
                .and_then(|p| Some(p.clone().data?.user_info?.membership_id)).unwrap_or_default();

            let future = members.iter()
                .map(|m| m.membership_id)
                .filter(|m| {
                    *m != membership_id
                })
                .map(|m| client.get_main_profile(m))
                .collect::<Vec<_>>();
            let future = futures::future::join_all(future);

            let profiles = future.await;
            let profiles = profiles.into_iter()
                .filter_map(|x| x.ok())
                .collect::<Vec<_>>();
            let profiles = vec![vec![profile.to_owned()], profiles].concat();
            Ok(profiles)

        })?
    };

    let template = match *fireteam {
        Ok(ref fireteam) => html! {
            <FireteamProvider fireteam={fireteam.to_owned()}>
                { for properties.children.iter() }
            </FireteamProvider>
        },
        Err(ref failure) => failure.to_string().into()
    };

    Ok(html!{
        <>
            { template }
        </>
    })
}
