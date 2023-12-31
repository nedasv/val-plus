use std::fs::File;
use std::io::Read;

use agent_select::*;
use iced::futures::lock;
use iced::widget::image::Handle;
use iced::{Element, Settings, Application, Command, Length, Renderer};
use iced::widget::{Button, button, column, Column, text, Row, container, Container, scrollable, Image, row};
use loader::{Agents, Agent, Tier};

mod loader;
mod auth;
mod party;
mod agent_select;
mod presence;

struct App {
    state: State,
    players: Option<Vec<Player>>,
    agents: Vec<Agent>,
    ranks: Vec<Tier>,
}

fn id_to_rank(id: u8) -> String {
    //let ranks: [&str; 10] = ["Iron 1", "Iron 2", "Iron 3", "Bronze"];


    return String::from("hello")
}

enum State { Loading, Party, PreGame, Game }

#[derive(Clone, Copy, Debug)]
enum Message {
    Refresh,
    LoadPreGamePlayers
}

fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size: (1280, 800),
            resizable:  false,
            ..Default::default()
        },
        ..Default::default()
    })
}

impl Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {

        let mut loader = loader::Loader::new().unwrap();

        println!("{:?}", loader);

        //loader.get_agents().unwrap();
        //let agents = loader.agent_cache.unwrap();

        (
            Self {
                state: State::Loading,
                players: None,
                agents: loader.agents,
                ranks: loader.ranks,
            }, Command::none()
        )

        // if let Ok(_) = loader.get_agents() {
        //     let agents = loader.agent_cache.unwrap();
        //     println!("{:?}", loader);

        //     return (
        //         Self {
        //             state: State::Loading, 
        //             players: None, 
        //             agents: agents,
        //          }
        //         ,Command::none()
        //     )

        //     //let agents = loader.agent_cache.unwrap().get_agent(uuid)

        //     //println!("{:?}", loader.agent_cache.unwrap().data)

        // } else {
        //     println!("UNSUCCESSFUL LOADING")
        // }

        // (
        //     Self {
        //         state: State::Loading, 
        //         players: None, 
        //         agents: None,
        //      }
        //     ,Command::none()
        // )
    }

    fn title(&self) -> String {
        String::from("Val+")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Refresh => {
                // let mut user = loader::User::default();

                // loader::get_region_shard(&mut user);
                

                // let lockfile = loader::get_lockfile().unwrap();
                // let auth = auth::get_auth(&lockfile).unwrap();


                // loader::get_player_info(&mut user, &auth);
                
                // let val_client = loader::get_client_version(&lockfile, &mut user).unwrap();

                // let party = party::get_party_id(&val_client.host_app, &user, &auth).unwrap();
                // let pre_game = agent_select::PreGameId::default().get_match_id(&user, &auth).unwrap();

                // agent_select::get_pre_game(&auth, &user, &pre_game);

                //println!("{:?}", party::get_party_members(&user, &party, &auth));
            }
            Message::LoadPreGamePlayers => {
                let mut user = loader::User::default();
                loader::get_region_shard(&mut user);

                let lockfile = loader::get_lockfile().unwrap();
                let auth = auth::get_auth(&lockfile).unwrap();

                loader::get_player_info(&mut user, &auth);

                let match_id = agent_select::PreGameId::default().get_match_id(&user, &auth).unwrap();
                let pre_game = agent_select::get_pre_game(&auth, &user, &match_id).unwrap();

                let pres = presence::Presence::new().load(&lockfile);

                self.players = Some(pre_game.ally_team.players);
                self.state = State::PreGame;
            }            
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {

        //let mut column: Column<'_, Message, Renderer> = Column::new();

        match &self.state {
            State::PreGame => {
                println!("PREGAME VIEW MATCH");

                let players = self.players.as_ref().unwrap();


                // if let Some(players) = &self.players {
                    let mut col = Column::new()
                        .height(Length::Fill)
                        .width(Length::Fill);
                    
                    for player in players {
                        let agent = self.agents.iter().find(|x| x.uuid.cmp(&player.agent_id).is_eq()).unwrap();
                        let image_link = agent.display_icon.clone();

                        let client = reqwest::blocking::Client::new();

                        if let Ok(resp) = client.get(&image_link).send() {

                            println!("RESP WAS OK");

                            let bytes = resp.bytes().unwrap();
                            let image = image::load_from_memory(&bytes).unwrap();
                            let byte = image.as_bytes().to_owned();
                            let handle = Handle::from_pixels(256, 256, byte);

                            let tiers = self.ranks.get(4).unwrap();
                            let rank = tiers.tiers.get(player.rank as usize).unwrap();

                            println!("{:?}", rank);

                            col = col.push(row!(Image::new(handle), text(format!("{}", player.rank)), Image::new(rank.get_image().unwrap())))
                        }
                    }

                    Container::new(
                        scrollable(
                            col
                        )
                    )
                    .height(Length::Fill)
                    .into()
            }
            _ => {
                button("Refresh").on_press(Message::LoadPreGamePlayers).into()
            }
        }

        // let mut content = Column::new();

        // for _i in 0..5 {
        //     content = content.push(button("hl"))
        // }

        // content.into()
        
        
        
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
