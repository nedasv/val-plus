use reqwest::header::{HeaderMap, HeaderValue};

use crate::{loader::{Lockfile, ValClient, User}, auth::Authorization};

#[derive(Debug, serde::Deserialize)]
pub struct PartyId {
    #[serde(rename = "CurrentPartyID")]
    party_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Party {
    #[serde(rename = "Members")]
    members: Vec<Member>
}

#[derive(Debug, serde::Deserialize)]
pub struct Member {
    #[serde(rename = "Subject")]
    puuid: String,
    #[serde(rename = "CompetitiveTier")]
    rank: u16,
    #[serde(rename = "PlayerIdentity")]
    player: PlayerIdentity,
}

#[derive(Debug, serde::Deserialize)]
pub struct PlayerIdentity {
    #[serde(rename = "PlayerCardID")]
    card_id: String,
    #[serde(rename = "PlayerTitleID")]
    title_id: String,
    #[serde(rename = "AccountLevel")]
    account_level: u16,
    #[serde(rename = "Incognito")]
    incognito: bool,
    #[serde(rename = "HideAccountLevel")]
    hide_level: bool,

}

pub fn get_party_id(val_client: &ValClient, user: &User, auth: &Authorization) -> Option<PartyId> {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return None,
    };

    let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/parties/v1/players/{}", &user.region, &user.shard, &user.puuid))
        .bearer_auth(&auth.access_token)
        .header("X-Riot-ClientVersion", &val_client.version)
        .header("X-Riot-Entitlements-JWT", &auth.token)
        .send() {
            Ok(resp) => resp,
            Err(_) => return None,
        };

    let party = match resp.json::<PartyId>() {
        Ok(party) => party,
        Err(_) => return None,
    };

    Some(party)
}

pub fn get_party_members(user: &User, party_id: &PartyId, auth: &Authorization) -> Option<Party> {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return None,
    };

    let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/parties/v1/parties/{}", &user.region, &user.shard, &party_id.party_id))
        .bearer_auth(&auth.access_token)
        .header("X-Riot-Entitlements-JWT", &auth.token)
        .send() {
            Ok(resp) => resp,
            Err(_) => return None,
    };

    let party = match resp.json::<Party>() {
        Ok(party) => party,
        Err(err) => {
            println!("{:?}", err);
            return None
        },
    };

    Some(party)



    //println!("{:?}", resp.text().unwrap())
}