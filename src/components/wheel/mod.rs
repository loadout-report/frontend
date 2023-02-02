use std::rc::Rc;
use log::info;
use rustgie::types::destiny::definitions::DestinyInventoryItemDefinition;
use rustgie::types::destiny::responses::DestinyProfileResponse;
use rustgie::types::destiny::TierType;
use stylist::style;
use stylist::yew::use_style;
use yew::prelude::*;
use yew::{utils};
use yew::{Children};
use yew::suspense::{use_future, UseFutureHandle};
use data::api::manifest::model::{Hash, Item};
use roll::Roll;
use crate::client::Client;
use crate::components::wheel::roll::_RollProps::roll_options;

pub mod roll;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RollOption {
    pub name: String,
    pub icon: String,
}

impl RollOption {
    fn new(name: String, icon: String) -> Self {
        RollOption {
            name,
            icon,
        }
    }
}

async fn get_roll_options(client: &Client) -> Result<Vec<RollOption>, anyhow::Error> {
    let mut special = vec![
        RollOption::new("Ghostbusters".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034233774503907480/unknown.png".to_string()),
        RollOption::new("Space Cowboys".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034226634376630355/unknown.png?width=457&height=676".to_string()),
        RollOption::new("Rat Pack".to_string(), "".to_string()),
        RollOption::new("Siva Crisis 2".to_string(), "".to_string()),
        RollOption::new("Drifter would be proud".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034228384475123712/unknown.png".to_string()),
        RollOption::new("Arrow to the Knee".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034217287030419456/unknown.png?width=886&height=676".to_string())
    ];
    let mut fetched_exotics: Vec<RollOption> = fetch_exotics(client)
        .await?
        .iter()
        .map(|item| RollOption::new(
            item.display_properties.as_ref().unwrap().name.clone().unwrap(),
            format!("https://www.bungie.net{}", item.display_properties.as_ref().unwrap().icon.clone().unwrap())
        ))
        .collect();
    special.append(&mut fetched_exotics);
    // info!("fetched exotics: {:?}", special);
    Ok(special)
}

async fn fetch_exotics(client: &Client) -> Result<Vec<DestinyInventoryItemDefinition>, anyhow::Error> {
    info!("fetching exotics");
    let items = client.get_items_definitions().await?;
    let mut exotics: Vec<DestinyInventoryItemDefinition> = items
        .iter()
        .filter(|(_, item)| item.inventory.as_ref().unwrap().tier_type == TierType::Exotic)
        .filter(|(_, item)| item.item_category_hashes.as_ref().unwrap().contains(&1))
        .map(|(_, item)| item.clone())
        .collect();
    return Ok(exotics);
}


#[derive(Clone, PartialEq, Eq, Debug, Properties)]
pub struct LoadedWheelProps {
    // pub players: Vec<String>,
}

// let other_roll_options: UseFutureHandle<Result<Vec<RollOption>, gloo_net::Error>> = use_future(|| async {
// let roll_options: Vec<RollOption> = get_roll_options().await?;
// Ok(roll_options)
// })?;

#[function_component(LoadedWheel)]
pub fn loaded_wheel(props: &LoadedWheelProps) -> HtmlResult {
    let client: Rc<Client> = use_context().expect("no client");

    let options: UseFutureHandle<Result<Vec<RollOption>, anyhow::Error>> = {
        let client = client.clone();
        use_future(|| async move {
            let client = client.clone();
            let options: Vec<RollOption> = get_roll_options(&client).await?;
            Ok(options)
        })?
    };

    let fireteam: Rc<Vec<DestinyProfileResponse>> = use_context::<Rc<Vec<DestinyProfileResponse>>>().unwrap();
    let template = match *options {
        Ok(ref res) => html! {
            <>
              {
                  fireteam.iter().map(|player| {
                    let player_name = player.profile.clone()
                        .and_then(|profile| profile.data)
                        .and_then(|data| data.user_info)
                        .map(|info| info.bungie_global_display_name.unwrap_or(info.display_name.unwrap_or_default()))
                        .unwrap();
                    html! {
                      <div key={player_name.to_owned()}>
                        <p>{player_name.to_owned()}</p>
                        <Roll roll_options={res.to_owned()} />
                      </div>
                    }
                  }).collect::<Html>()
              }
            </>
        },
        Err(ref failure) => failure.to_string().into()
    };

    Ok(html! {
        <>
            {template}
        </>
    })
}

#[derive(Clone, PartialEq, Eq, Debug, Properties)]
struct RollOptionProviderProps {
    roll_options: Vec<RollOption>,
}

#[function_component(RollOptionProvider)]
fn roll_option_provider(props: &RollOptionProviderProps) -> Html {
    info!("roll options: {:?}", props.roll_options);
    let options = use_memo(|_| {
        props.roll_options.to_owned()
    }, ());

    html! {
        // <ContextProvider<Rc<Vec<RollOption>>> context={options}>
            //<Roll roll_options={props.roll_options} />
        // </ContextProvider<Rc<Vec<RollOption>>>>
    }
}
