

#[derive(serde::Deserialize, Debug)]
pub struct Agents {
    pub data: Vec<Agent>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Agent {
    pub uuid: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "displayIcon")]
    pub  display_icon: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Ranks {
    #[serde(rename = "data")]
    pub data: Vec<Tier>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Tier {
    #[serde(rename = "tiers")]
    pub tiers: Vec<Rank>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Rank {
    #[serde(rename = "tier")]
    pub rank: u8,
    #[serde(rename = "tierName")]
    pub rank_name: String,
    #[serde(rename = "smallIcon")]
    pub small_icon_link: Option<String>,
}

#[derive(Debug)]
pub struct Loader {
    pub agents: Vec<Agent>,
    pub ranks: Vec<Tier>
}

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

#[derive(Debug)]
pub enum LoaderError {
    NoLockFile, NoShard, NoVersion, ClientError, NoPlayerInfo
}

impl Default for User {
    fn default() -> Self {
        Self {
            region: String::from("eu"),
            shard: String::from("eu"),
            puuid: String::from("0"),
        }
    }
}

// Reads lockfile data from "C:\Users\User1\AppData\Local\Riot Games\Riot Client\Config" which contains the port and password to access local api
pub fn get_lockfile() -> Result<(String, String), LoaderError> {
    if let Ok(path) = std::env::var("LOCALAPPDATA") {
        let lockfile_path = format!{"{}{}", path, "\\Riot Games\\Riot Client\\Config\\lockfile"};
        
        let content = match std::fs::read_to_string(&lockfile_path) {
            Ok(text) => text,
            Err(_) => return Err(LoaderError::ClientError),
        };

        let split_content: Vec<&str> = content.split(":").collect();

        if let Some(port) = split_content.get(2) {
            if let Some(password) = split_content.get(3) {
                return Ok((port.to_string(), password.to_string()))
            }
        }
    }

    Err(LoaderError::NoLockFile)
}

pub fn get_region_shard() -> Result<(String, String), LoaderError>{
    if let Ok(path) = std::env::var("LOCALAPPDATA") {
        let shooter_game_path = format!("{}{}", path, "\\VALORANT\\Saved\\Logs\\ShooterGame.log");

        let content = match std::fs::read_to_string(&shooter_game_path) {
            Ok(text) => text,
            Err(_) => return Err(LoaderError::ClientError),
        };

        // Uses an endpoint log used by valorant to extract region and shard of player
        let split_1: Vec<&str> = content.split("[Party_FetchCustomGameConfigs], URL [GET ").collect();
        let split_2: Vec<&str> = split_1.get(1).unwrap().split("/parties/v1/parties/customgameconfigs]").collect();

        let link = match split_2.get(0) {
            Some(link) => link,
            None => return Err(LoaderError::NoShard),
        };

        let region_re = regex::Regex::new(r"-(\w+)-").unwrap();
        let region = match region_re.captures(&link) {
            Some(region) => region,
            None => return Err(LoaderError::NoShard),
        };

        let shard_re = regex::Regex::new(r"1.(\w+).").unwrap();
        let shard = match shard_re.captures(&link) {
            Some(shard) => shard,
            None => return Err(LoaderError::NoShard),
        };

        if let Some(region) = region.get(1) {
            if let Some(shard) = shard.get(1) {
                return Ok((region.as_str().to_string(), shard.as_str().to_string()))
            }
        }
    }

    return Err(LoaderError::ClientError)
}

pub fn get_client_version(port: String, password: String) -> Result<String, LoaderError> {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return Err(LoaderError::ClientError),
    };

    let res = match client.get(format!("https://127.0.0.1:{}/product-session/v1/external-sessions", &port)).basic_auth("riot", Some(&password)).send() {
        Ok(response) => {
            println!("{:?}", response);
            response
        },
        Err(err) => {
            println!("{:?}", err);    
            return Err(LoaderError::NoVersion);
        },
    };

    let map = res.json::<HostApp>().unwrap();

    return Ok(map.host_app.version.clone());
}

pub fn get_player_info(access_token: String) -> Result<String, LoaderError> {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return Err(LoaderError::ClientError),
    };

    let res = match client.get("https://auth.riotgames.com/userinfo").bearer_auth(&access_token).send() {
        Ok(response) => {
            println!("{:?}", response);
            response
        },
        Err(err) => {
            println!("{:?}", err);    
            return Err(LoaderError::NoPlayerInfo);
        },
    };

    let info = res.json::<UserId>().unwrap();

    return Ok(info.puuid.clone());
}
