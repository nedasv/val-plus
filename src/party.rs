use reqwest::header::{HeaderMap, HeaderValue};

use crate::{loader::{Lockfile, ValClient, User}, auth::Authorization};

#[derive(Debug, serde::Deserialize)]
pub struct Party {
    #[serde(rename = "CurrentPartyID")]
    party_id: String,
}

pub fn get_party(lockfile: &Lockfile, val_client: &ValClient, user: &User, auth: &Authorization) {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return,
    };

    let mut headers = HeaderMap::new();

    let resp = client.get(format!("https://glz-{}-1.{}.a.pvp.net/parties/v1/players/{}", &user.region, &user.shard, &user.puuid))
        .bearer_auth(&auth.access_token)
        .header("X-Riot-ClientVersion", &val_client.version)
        .header("X-Riot-Entitlements-JWT", &auth.token)
        .send();

    println!("{:?}", resp.unwrap().text().unwrap())
}