use crate::auth::Authorization;


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
pub fn get_lockfile() -> Option<Lockfile> {
    if let Ok(path) = std::env::var("LOCALAPPDATA") {
        let lockfile_path = format!{"{}{}", path, "\\Riot Games\\Riot Client\\Config\\lockfile"};
        
        let content = match std::fs::read_to_string(&lockfile_path) {
            Ok(text) => text,
            Err(_) => return None,
        };

        let split_content: Vec<&str> = content.split(":").collect();
        let mut lockfile = Lockfile::default();

        if let Some(port) = split_content.get(2) {
            lockfile.port = port.to_string();
        } else {
            return None;
        }

        if let Some(password) = split_content.get(3) {
            lockfile.password = password.to_string();
        } else {
            return None;
        }
        
        return Some(lockfile);
    }
    return None;
}

pub fn get_region_shard(user: &mut User) -> Option<bool>{
    println!("REGION");

    if let Ok(path) = std::env::var("LOCALAPPDATA") {
        let shooter_game_path = format!("{}{}", path, "\\VALORANT\\Saved\\Logs\\ShooterGame.log");

        println!("{:?}", shooter_game_path);

        let content = match std::fs::read_to_string(&shooter_game_path) {
            Ok(text) => text,
            Err(err) => {
                println!("{:?}", err);
                return None},
        };

        // Uses an endpoint log used by valorant to extract region and shard of player
        let split_1: Vec<&str> = content.split("[Party_FetchCustomGameConfigs], URL [GET ").collect();
        let split_2: Vec<&str> = split_1.get(1).unwrap().split("/parties/v1/parties/customgameconfigs]").collect();

        let link = match split_2.get(0) {
            Some(link) => link,
            None => return None,
        };

        let region_re = regex::Regex::new(r"-(\w+)-").unwrap();
        let region = match region_re.captures(&link) {
            Some(region) => region,
            None => return None,
        };

        let shard_re = regex::Regex::new(r"1.(\w+).").unwrap();
        let shard = match shard_re.captures(&link) {
            Some(shard) => shard,
            None => return None,
        };

        if let Some(region) = region.get(1) {
            user.region = region.as_str().to_string();

            if let Some(shard) = shard.get(1) {
                user.shard = shard.as_str().to_string();
            } else {
                return None;
            }
        } else {
            return None;
        }
        
        //println!("{:?} {:?}", region.get(1).unwrap().as_str().to_string(), shard.get(1).unwrap().as_str().to_string());
    }

    return None
}

pub fn get_client_version(lockfile: &Lockfile, user: &mut User) -> Option<HostApp> {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return None,
    };

    let res = match client.get(format!("https://127.0.0.1:{}/product-session/v1/external-sessions", &lockfile.port)).basic_auth("riot", Some(&lockfile.password)).send() {
        Ok(response) => {
            println!("{:?}", response);
            response
        },
        Err(err) => {
            println!("{:?}", err);    
            return None;
        },
    };

    return Some(res.json::<HostApp>().unwrap());
}

pub fn get_player_info(user: &mut User, auth: &Authorization) {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return,
    };

    let res = match client.get("https://auth.riotgames.com/userinfo").bearer_auth(&auth.access_token).send() {
        Ok(response) => {
            println!("{:?}", response);
            response
        },
        Err(err) => {
            println!("{:?}", err);    
            return;
        },
    };

    let info = res.json::<UserId>().unwrap();
    user.puuid = info.puuid.clone();
}