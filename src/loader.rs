use regex::Regex;
use reqwest::blocking::Client;

#[derive(Debug, Default)]
pub struct Lockfile {
    pub port: String,
    pub password: String,
}

#[derive(Debug)]
pub struct User {
    pub region: String,
    pub shard: String,
    pub puuid: String,
}

#[derive(serde::Deserialize)]
pub struct UserId {
    #[serde(rename = "sub")]
    pub puuid: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct HostApp {
    #[serde(rename = "host_app")]
    pub host_app: ValClient,
}

#[derive(Debug, serde::Deserialize)]
pub struct ValClient {
    #[serde(rename = "version")]
    pub version: String,
}

pub struct Loader {
    client: Client,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap()
        }
    }

    pub fn get_port_and_password(&self) -> Option<(String, String)> {
        match std::env::var("LOCALAPPDATA") {
            Ok(path) => {
                match std::fs::read_to_string(format!("{}{}", path, "\\Riot Games\\Riot Client\\Config\\lockfile")) {
                    Ok(lockfile) => {
                        let lock_split: Vec<&str> = lockfile.split(":").collect();

                        Some((lock_split.get(2).unwrap().to_string(), lock_split.get(3).unwrap().to_string()))

                    }
                    Err(_) => return None,
                }
            }
            Err(_) => return None,
        }
    }

    pub fn get_region_and_shard(&self) -> Option<(String, String)> {
        match std::env::var("LOCALAPPDATA") {
            Ok(path) => {
                match std::fs::read_to_string(format!("{}{}", path, "\\VALORANT\\Saved\\Logs\\ShooterGame.log")) {
                    Ok(shooter_game) => {
                        let re = Regex::new(r"https://glz-(.+?)-1.(.+?).a.pvp.net").unwrap();

                        if let Some(capture) = re.captures(&shooter_game) {
                            if let (Some(region), Some(shard)) = (capture.get(1), capture.get(2)) {
                                return Some((region.as_str().to_string(), shard.as_str().to_string()))
                            }
                        }

                    }
                    Err(_) => return None,
                }
            }
            Err(_) => return None,
        }

        return None
    }

    pub fn get_client_version(&self, port: String, password: String) -> Option<String> {
        return match self.client.get(format!("https://127.0.0.1:{}/product-session/v1/external-sessions", &port)).basic_auth("riot", Some(&password)).send() {
            Ok(res) => {
                match res.json::<HostApp>() {
                    Ok(json) => Some(json.host_app.version),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    pub fn get_player_info(&self, access_token: String) -> Option<String> {
        return match self.client.get("https://auth.riotgames.com/userinfo").bearer_auth(&access_token).send() {
            Ok(res) => {
                match res.json::<UserId>() {
                    Ok(json) => Some(json.puuid),
                    Err(_) => None,
                }
            }
            Err(_) => None
        }
    }
}
