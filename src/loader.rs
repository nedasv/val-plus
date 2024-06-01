use regex::Regex;
use reqwest::blocking::Client;

#[derive(Debug, Default)]
pub struct Lockfile {
    pub port: String,
    pub password: String,
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

#[derive(Debug, Default, serde::Deserialize)]
pub struct Authorization {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "token")]
    pub token: String,
}

#[derive(Debug)]
pub enum LoaderError {
    Auth, PortPassword, RegionShard, ClientVersion, PlayerInfo, NotLoaded
}

#[derive(Debug, Default)]
pub struct Loader {
    client: Client,

    pub port: String,
    pub password: String,
    pub region: String,
    pub shard: String,
    pub access_token: String,
    pub token: String,
    pub client_version: String,
    pub puuid: String,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            ..Default::default()
        }
    }

    pub fn try_load(&mut self) -> Result<(), LoaderError> {
        self.get_port_and_password()?;
        self.get_auth()?;
        self.get_region_and_shard()?;
        self.get_client_version()?;
        self.get_player_info()?;

        Ok(())
    }

    pub fn get_auth(&mut self) -> Result<(), LoaderError> {
        match self.client.get(format!("https://127.0.0.1:{}/entitlements/v1/token", self.port)).basic_auth("riot", Some(&self.password)).send() {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<Authorization>() {
                        Ok(auth) => {
                            self.token = auth.token.clone();
                            self.access_token = auth.access_token.clone();

                            return Ok(())
                        }
                        _ => {},
                    }
                }
            }
            _ => {},
        }

        Err(LoaderError::Auth)
    }

    pub fn get_port_and_password(&mut self) -> Result<(), LoaderError> {
        match std::env::var("LOCALAPPDATA") {
            Ok(path) => {
                match std::fs::read_to_string(format!("{}{}", path, "\\Riot Games\\Riot Client\\Config\\lockfile")) {
                    Ok(lockfile) => {
                        let lock_split: Vec<&str> = lockfile.split(":").collect();

                        self.port = lock_split.get(2).unwrap().to_string();
                        self.password = lock_split.get(3).unwrap().to_string();

                        return Ok(())

                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Err(LoaderError::PortPassword)
    }

    pub fn get_region_and_shard(&mut self) -> Result<(), LoaderError> {
        match std::env::var("LOCALAPPDATA") {
            Ok(path) => {
                match std::fs::read_to_string(format!("{}{}", path, "\\VALORANT\\Saved\\Logs\\ShooterGame.log")) {
                    Ok(shooter_game) => {
                        let re = Regex::new(r"https://glz-(.+?)-1.(.+?).a.pvp.net").unwrap();

                        if let Some(capture) = re.captures(&shooter_game) {
                            if let (Some(region), Some(shard)) =
                                (capture.get(1), capture.get(2)) {

                                self.region = region.as_str().to_string();
                                self.shard = shard.as_str().to_string();

                                return Ok(())
                            }
                        }

                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Err(LoaderError::RegionShard)
    }

    pub fn get_client_version(&mut self) -> Result<(), LoaderError> {
        match self.client.get(format!("https://127.0.0.1:{}/product-session/v1/external-sessions", &self.port)).basic_auth("riot", Some(&self.password)).send() {
            Ok(res) => {
                match res.json::<HostApp>() {
                    Ok(json) => {
                        self.client_version = json.host_app.version;

                        return Ok(())
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Err(LoaderError::ClientVersion)
    }

    pub fn get_player_info(&mut self) -> Result<(), LoaderError> {
        match self.client.get("https://auth.riotgames.com/userinfo").bearer_auth(&self.access_token).send() {
            Ok(res) => {
                match res.json::<UserId>() {
                    Ok(json) => {
                        self.puuid = json.puuid;

                        return Ok(())
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Err(LoaderError::PlayerInfo)
    }
}
