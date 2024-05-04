use serde::Deserialize;
use crate::RiotAuth;

#[derive(Deserialize, Debug)]
pub struct NameService {
    #[serde(rename = "Subject")]
    pub uuid: String,
    #[serde(rename = "GameName")]
    pub game_name: String,
    #[serde(rename = "TagLine")]
    pub tag_line: String,
}

impl NameService {
    pub fn get_names(auth: &RiotAuth, users: Vec<String>) -> Option<Vec<NameService>> {
        let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build() {
            Ok(client) => client,
            Err(_) => return None,
        };

        return match client.put(format!("https://pd.{}.a.pvp.net/name-service/v2/players", &auth.shard))
            .bearer_auth(&auth.access_token)
            .header("X-Riot-Entitlements-JWT", &auth.token)
            .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
            .header("X-Riot-ClientVersion", &auth.client_ver)
            .body(serde_json::to_string(&users).unwrap())
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<Vec<NameService>>() {
                        Ok(json) => Some(json),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            },
            Err(_) => None,
        }
    }
}
