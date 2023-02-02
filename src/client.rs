use std::collections::HashMap;
use anyhow::Context;
use log::info;
use rustgie::{RustgieClient, RustgieClientBuilder};
use rustgie::types::api_response_::BungieApiResponse;
use rustgie::types::BungieMembershipType;
use rustgie::types::destiny::config::DestinyManifest;
use rustgie::types::destiny::definitions::collectibles::DestinyCollectibleDefinition;
use rustgie::types::destiny::definitions::DestinyInventoryItemDefinition;
use rustgie::types::destiny::DestinyComponentType;
use rustgie::types::destiny::responses::{DestinyLinkedProfilesResponse, DestinyProfileResponse};
use rustgie::types::user::{ExactSearchRequest, UserInfoCard};

use serde_derive::{Serialize, Deserialize};
use data::api::manifest::model::Item;

const API_BASE: &str = env!("API_BASE");

#[derive(Clone, Debug)]
pub struct Client {
    pub api_key: String,
}

impl PartialEq for Client {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

pub fn rc(api_key: &str) -> RustgieClient {
    rustgie::RustgieClientBuilder::new().with_api_key(api_key).build().unwrap()
}

impl Client {
    pub fn new() -> Self {
        Self {
            api_key: String::from("")
        }
    }

    pub fn with_api_key(api_key: &str) -> Self {
        Self {
            api_key: String::from(api_key)
        }
    }

    pub async fn search(&self, search_request: ExactSearchRequest) -> Result<Vec<UserInfoCard>, anyhow::Error> {
        info!("searching for user");
        rc(&self.api_key).destiny2_search_destiny_player_by_bungie_name(BungieMembershipType::All, search_request, None).await
    }

    pub async fn get_manifest(&self) -> Result<DestinyManifest, anyhow::Error> {
        let client = rc(&self.api_key);
        client.destiny2_get_destiny_manifest(None).await
    }

    pub async fn get_items_definitions(&self) -> Result<HashMap<String, DestinyInventoryItemDefinition>, anyhow::Error> {
        info!("getting item definitions");
        let manifest = self.get_manifest().await?;
        let content_paths = manifest.json_world_component_content_paths.unwrap();
        let item_definitions_ref = content_paths.get("en").unwrap().get("DestinyInventoryItemDefinition").unwrap();
        let item_definitions_url = format!("https://www.bungie.net{}", item_definitions_ref);
        reqwest::get(&item_definitions_url)
            .await.context("couldn't fetch manifest data")?.json::<HashMap<String, DestinyInventoryItemDefinition>>()
            .await.context("couldn't parse manifest data")
    }

    pub async fn get_items(&self) -> Result<Vec<DestinyInventoryItemDefinition>, anyhow::Error> {
        let mut items = self.get_items_definitions().await?;
        let mut items: Vec<_> = items.values_mut()
            .filter(|i| i.item_category_hashes.is_none() || !i.item_category_hashes.as_ref().unwrap().contains(&3109687656))
            .map(|i| i.clone())
            .collect();
        items.sort_by_key(|i| i.display_properties.as_ref().unwrap().name.clone());
        Ok(items)
    }

    pub async fn get_collectibles(&self) -> Result<Vec<DestinyCollectibleDefinition>, anyhow::Error> {
        info!("getting collectibles");
        let manifest = self.get_manifest().await?;
        let content_paths = manifest.json_world_component_content_paths.unwrap();
        let collectible_ref = content_paths.get("en").unwrap().get("DestinyCollectibleDefinition").unwrap();
        let collectible_url = format!("https://www.bungie.net{}", collectible_ref);
        reqwest::get(&collectible_url).await.context("couldn't fetch manifest data")?.json().await.context("couldn't parse manifest data")
    }

    pub async fn get_profile(&self, membership_type: i32, membership_id: i64) -> Result<DestinyProfileResponse, anyhow::Error> {
        info!("getting profile");
        rc(&self.api_key).destiny2_get_profile(
            membership_id,
            parse_membership_type(membership_type).unwrap(),
            Some(vec![
                DestinyComponentType::Profiles,
                DestinyComponentType::Characters,
                DestinyComponentType::CharacterEquipment,
                DestinyComponentType::ItemInstances,
                DestinyComponentType::ItemPerks,
                DestinyComponentType::ItemStats,
                DestinyComponentType::Collectibles,
                DestinyComponentType::Transitory,
            ]), None).await
    }

    pub async fn get_linked_profiles(&self, membership_id: i64) -> Result<DestinyLinkedProfilesResponse, anyhow::Error> {
        info!("fuck");
        rc(&self.api_key)
            .destiny2_get_linked_profiles(membership_id, BungieMembershipType::All, Some(true), None)
            .await
    }

    pub async fn get_main_profile(&self, membership_id: i64) -> Result<DestinyProfileResponse, anyhow::Error> {
        info!("getting main profile");
        let linked_profiles = self.get_linked_profiles(membership_id).await?;
        let linked_profiles = linked_profiles.profiles.unwrap();
        let membership_type = if linked_profiles.len() > 1 {
            linked_profiles.iter().find(|p| p.is_cross_save_primary).unwrap().membership_type
        } else {
            linked_profiles[0].membership_type
        };
        self.get_profile(membership_type as i32, membership_id).await
    }

}


fn parse_membership_type(t: i32) -> Option<BungieMembershipType> {
    match t {
        0 => Some(BungieMembershipType::None),
        1 => Some(BungieMembershipType::TigerXbox),
        2 => Some(BungieMembershipType::TigerPsn),
        3 => Some(BungieMembershipType::TigerSteam),
        4 => Some(BungieMembershipType::TigerBlizzard),
        5 => Some(BungieMembershipType::TigerStadia),
        10 => Some(BungieMembershipType::TigerDemon),
        254 => Some(BungieMembershipType::BungieNext),
        -1 => Some(BungieMembershipType::All),
        _ => None,
    }
}
