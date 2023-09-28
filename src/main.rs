use std::fs::File;
use std::io::Read;

use iced::futures::lock;
use iced::{Element, Sandbox, Settings};
use iced::widget::{Button, button};

mod loader;
mod auth;
mod party;

struct App;
#[derive(Clone, Copy, Debug)]
enum Message {
    GetData,
}

fn main() -> iced::Result {
    App::run(Settings::default())
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Val+")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GetData => {
                let mut user = loader::User::default();

                loader::get_region_shard(&mut user);
                

                let lockfile = loader::get_lockfile().unwrap();
                let auth = auth::get_auth(&lockfile).unwrap();


                loader::get_player_info(&mut user, &auth);
                
                let val_client = loader::get_client_version(&lockfile, &mut user).unwrap();

                let party = party::get_party_id(&val_client.host_app, &user, &auth).unwrap();

                println!("{:?}", party::get_party_members(&user, &party, &auth));


            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        button("Get Data").on_press(Message::GetData).into()
    }
}

// #[derive(Debug, Default)]
// struct Lockfile {
//     port: String,
//     password: String,
// }

// #[derive(Default, serde::Deserialize)]
// struct Entiltement {
//     #[serde(rename = "accessToken")]
//     access_token: String,
//     token: String,
// }

// #[derive(serde::Deserialize, Debug, Default)]
// struct User {
//     #[serde(rename = "sub")]
//     uuid: String,
// }

// #[derive(serde::Deserialize, Debug)]
// struct GamePlayer {
//     #[serde(rename = "MatchID")]
//     match_id: String,
// }

// #[derive(serde::Deserialize, Debug)]
// struct CurrentMatch {
//     #[serde(rename = "MapID")]
//     map_id: String,
//     #[serde(rename = "ModeID")]
//     mode_id: String,
//     #[serde(rename = "ProvisioningFlow")]
//     game_type: String,
//     #[serde(rename = "Players")]
//     players: Vec<CurrentMatchPlayer>,
// }

// #[derive(serde::Deserialize, Debug)]
// struct CurrentMatchPlayer {
//     #[serde(rename = "TeamID")]
//     team_id: String,
//     #[serde(rename = "CharacterID")]
//     character_id: String,
//     #[serde(rename = "PlayerIdentity")]
//     player_identity: PlayerIdentity,
//     #[serde(rename = "SeasonalBadgeInfo")]
//     player_act_info: PlayerActInfo,

// }

// #[derive(serde::Deserialize, Debug)]
// struct PlayerIdentity {
//     #[serde(rename = "Subject")]
//     uuid: String,
//     #[serde(rename = "PlayerCardID")]
//     card_id: String,
//     #[serde(rename = "PlayerTitleID")]
//     title_id: String,
//     #[serde(rename = "AccountLevel")]
//     account_level: u16,
//     #[serde(rename = "Incognito")]
//     incognito: bool,
//     #[serde(rename = "HideAccountLevel")]
//     hide_account_level: bool,
// }

// #[derive(serde::Deserialize, Debug)]
// struct PlayerActInfo {
//     #[serde(rename = "NumberOfWins")]
//     wins: u16,
//     #[serde(rename = "Rank")]
//     rank: u8,
//     #[serde(rename = "LeaderboardRank")]
//     leaderboard_rank: u16,
// }

// #[tokio::main]
// async fn main() {

//     let mut lockfile = Lockfile::default();
//     let mut user = User::default();

//     //let lockfile = std::env::var("LOCALAPPDATA");

//     if let Ok(path) = std::env::var("LOCALAPPDATA") {
//         let lockfile_path = format!{"{}{}", path, "\\Riot Games\\Riot Client\\Config\\lockfile"};

//         let content = match std::fs::read_to_string(&lockfile_path) {
//             Ok(text) => text,
//             Err(_) => return,
//         };

//         let split_content: Vec<&str> = content.split(":").collect();

//         lockfile.port = split_content.get(2).unwrap().to_string();
//         lockfile.password = split_content.get(3).unwrap().to_string();
//     }

//     //println!("{:?}", lockfile);

//     let client = reqwest::Client::builder()
//         .danger_accept_invalid_certs(true) // Local ip does not have ssl certificate
//         .build()
//         .unwrap();

//     let result =  match client.get(format!("https://127.0.0.1:{}/entitlements/v1/token", lockfile.port))
//         .basic_auth("riot", Some(lockfile.password))
//         .send()
//         .await {
//             Ok(resp) => {
//                 //let mut text = String::new();

//                 if let Ok(content) = resp.json::<Entiltement>().await {
//                     content
//                     //text = content;
//                 } else {
//                     return;
//                 }

//                 //text
//             },
//             Err(_) => return,
//     };

//     let player_info_res = client.get("https://auth.riotgames.com/userinfo")
//         .bearer_auth(&result.access_token)
//         .send()
//         .await;

//     let user_puuid = player_info_res.unwrap().json::<User>().await.unwrap();

//     let player_match = client.get(format!("https://glz-eu-1.eu.a.pvp.net/core-game/v1/players/{}", user_puuid.uuid))
//         .bearer_auth(&result.access_token)
//         .header("X-Riot-Entitlements-JWT", &result.token)
//         .send()
//         .await;

//     let game_player = player_match.unwrap().json::<GamePlayer>().await.unwrap();

//     let match_data = client.get(format!("https://glz-eu-1.eu.a.pvp.net/core-game/v1/matches/{}", game_player.match_id))
//         .bearer_auth(&result.access_token)
//         .header("X-Riot-Entitlements-JWT", &result.token)
//         .send()
//         .await;

//     println!("{:?}", match_data.unwrap().json::<CurrentMatch>().await.unwrap());
// }
