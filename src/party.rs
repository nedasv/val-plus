// use crate::loader::{Lockfile, ValClient, User};

// #[derive(Debug, serde::Deserialize)]
// pub struct Party {
//     #[serde(rename = "CurrentPartyID")]
//     party_id: String,
// }

// pub fn get_party(lockfile: &Lockfile, val_client: &ValClient, user: &User) {
//     let client = match reqwest::blocking::Client::builder()
//         .danger_accept_invalid_certs(true)
//         .build() {
//             Ok(client) => client,
//             Err(_) => return Err(Response::ClientNotBuilt),
//     };

//     let resp = client.get(format!("https://glz-{}-1.{}.a.pvp.net/parties/v1/players/{}", &user.region, &user.shard, &));
// }