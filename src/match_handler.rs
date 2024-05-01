// use std::time::{SystemTime, UNIX_EPOCH};
// use reqwest::StatusCode;
// use crate::{RiotAuth, LoadedPlayer, name_service, TeamType};
// use crate::database;
// use crate::database::{MatchHistory, NameHistory};
//
// // pub enum MatchHandlerError {
// //     ClientError, ResponseError(Option<StatusCode>)
// // }
// //
// // #[derive(Debug)]
// // pub struct MatchHandler {
// //     pub match_id: String,
// //     pub players: Vec<Player>,
// // }
// //
// // impl MatchHandler {
// //
// //     // pub fn get_match_details(match_id: String) {
// //     //     let client = match reqwest::blocking::Client::builder()
// //     //         .danger_accept_invalid_certs(true)
// //     //         .build() {
// //     //         Ok(client) => client,
// //     //         Err(_) => return Err(Error::ClientError),
// //     //     };
// //     //
// //     //     let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}", auth.region, auth.shard, match_id))
// //     //         .bearer_auth(&auth.access_token)
// //     //         .header("X-Riot-Entitlements-JWT", &auth.token)
// //     //         .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
// //     //         .header("X-Riot-ClientVersion", &auth.client_ver)
// //     //         .send() {
// //     //         Ok(resp) => resp,
// //     //         Err(err) => {
// //     //             println!("{:?}", err);
// //     //             return Err(Error::RiotError)
// //     //         }
// //     //     };
// //     //
// //     //     if resp.status().is_success() {
// //     //
// //     //         //println!("{:?}", resp.json::<CurrentGameMatch>());
// //     //
// //     //         return if let Ok(pre_game) = resp.json::<CurrentGameMatch>() {
// //     //             Ok(pre_game)
// //     //         } else {
// //     //             Err(Error::NotPreGame)
// //     //         }
// //     //     }
// //     //
// //     //     println!("{:?}", resp);
// //     //
// //     //     return Err(Error::Unknown)
// //     // }
// //
// //     pub fn get_match_id(region: String, shard: String, puuid: String, access_token: String, token: String, client_ver: String) -> Result<String, MatchHandlerError> {
// //         let client = match reqwest::blocking::Client::builder()
// //             .danger_accept_invalid_certs(true)
// //             .build() {
// //             Ok(client) => client,
// //             Err(_) => return Err(MatchHandlerError::ClientError),
// //         };
// //
// //         let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/players/{}", region, shard, puuid))
// //             .bearer_auth(access_token)
// //             .header("X-Riot-Entitlements-JWT", token)
// //             .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
// //             .header("X-Riot-ClientVersion", client_ver)
// //             .send() {
// //             Ok(resp) => resp,
// //             Err(err) => {
// //                 return Err(MatchHandlerError::ResponseError(err.status()))
// //             }
// //         };
// //
// //         if resp.status().is_success() {
// //             return match resp.json::<CurrentGamePlayer>() {
// //                 Ok(resp) => {
// //                     Ok(resp.match_id)
// //                 }
// //                 Err(err) => {
// //                     Err(MatchHandlerError::ResponseError(err.status()))
// //                 }
// //             }
// //         }
// //
// //         return Err(MatchHandlerError::ClientError)
// //     }
// // }
//
// #[derive(serde::Deserialize, Debug, Default)]
// pub struct CurrentGamePlayer {
//     #[serde(rename = "MatchID")]
//     pub match_id: String,
// }
//
// #[derive(serde::Deserialize, Debug, Default)]
// pub struct CurrentGameMatch {
//     #[serde(rename = "Players")]
//     pub players: Vec<Player>,
// }
//
// #[derive(serde::Deserialize, Debug, Default)]
// pub struct Player {
//     #[serde(rename = "TeamID")]
//     pub team_id: String,
//     #[serde(rename = "CharacterID")]
//     pub agent_id: String,
//     #[serde(rename = "PlayerIdentity")]
//     pub player_identity: PlayerIdentity,
// }
//
// #[derive(serde::Deserialize, Debug, Default)]
// pub struct PlayerIdentity {
//     #[serde(rename = "Subject")]
//     pub uuid: String,
//     #[serde(rename = "PlayerCardID")]
//     // pub card_id: String,
//     // #[serde(rename = "PlayerTitleID")]
//     // pub title_id: String,
//     #[serde(rename = "AccountLevel")]
//     pub level: u16,
//     #[serde(rename = "Incognito")]
//     pub incognito: bool,
//     #[serde(rename = "HideAccountLevel")]
//     pub hide_level: bool,
// }
//
// #[derive(Debug)]
// pub enum Error { NotPreGame, ClientError, RiotError, Unknown }
//
// impl CurrentGamePlayer {
//     pub fn get_match_id(auth: &RiotAuth) -> Result<String, Error> {
//         let client = match reqwest::blocking::Client::builder()
//             .danger_accept_invalid_certs(true)
//             .build() {
//             Ok(client) => client,
//             Err(_) => return Err(Error::ClientError),
//         };
//
//         let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/players/{}", auth.region, auth.shard, auth.puuid))
//             .bearer_auth(&auth.access_token)
//             .header("X-Riot-Entitlements-JWT", &auth.token)
//             .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
//             .header("X-Riot-ClientVersion", &auth.client_ver)
//             .send() {
//             Ok(resp) => resp,
//             Err(err) => {
//                 println!("{:?}", err);
//                 return Err(Error::RiotError)
//             }
//         };
//
//         if resp.status().is_success() {
//             return if let Ok(pre_game) = resp.json::<CurrentGamePlayer>() {
//                 Ok(pre_game.match_id.clone())
//             } else {
//                 Err(Error::NotPreGame)
//             }
//         }
//
//         println!("{:?}", resp.text());
//
//         return Err(Error::Unknown)
//     }
//
//     pub fn get_match(auth: &RiotAuth, match_id: String) -> Result<CurrentGameMatch, Error> {
//         let client = match reqwest::blocking::Client::builder()
//             .danger_accept_invalid_certs(true)
//             .build() {
//             Ok(client) => client,
//             Err(_) => return Err(Error::ClientError),
//         };
//
//         let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}", auth.region, auth.shard, match_id))
//             .bearer_auth(&auth.access_token)
//             .header("X-Riot-Entitlements-JWT", &auth.token)
//             .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
//             .header("X-Riot-ClientVersion", &auth.client_ver)
//             .send() {
//             Ok(resp) => resp,
//             Err(err) => {
//                 println!("{:?}", err);
//                 return Err(Error::RiotError)
//             }
//         };
//
//         if resp.status().is_success() {
//
//             //println!("{:?}", resp.json::<CurrentGameMatch>());
//
//             return if let Ok(pre_game) = resp.json::<CurrentGameMatch>() {
//                 Ok(pre_game)
//             } else {
//                 Err(Error::NotPreGame)
//             }
//         }
//
//         println!("{:?}", resp);
//
//         return Err(Error::Unknown)
//     }
//
//     pub fn get_players(auth: RiotAuth, prev_match_id: String) -> Result<(Vec<LoadedPlayer>, String), Error> {
//         let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
//
//         if let Ok(match_id) = self::CurrentGamePlayer::get_match_id(&auth) {
//
//             if prev_match_id == match_id {
//                 println!("same match id");
//                 return Err(Error::NotPreGame)
//             }
//
//
//             if let Ok(current_match) = self::CurrentGamePlayer::get_match(&auth, match_id.clone()) {
//                 let player_ids: Vec<String> = current_match.players.iter().map(|player| player.player_identity.uuid.clone()).collect();
//
//
//                 if let Ok(player_names) = name_service::NameService::get_names(&auth, player_ids) {
//                     let mut players = Vec::new();
//                     let mut times_played: i64 = 0;
//                     let mut last_played: i64 = time_now;
//                     let mut match_history: Option<Vec<MatchHistory>> = None;
//                     let mut name_history: Option<Vec<NameHistory>> = None;
//
//                     for (i, name) in player_names.iter().enumerate() {
//                         if let Ok(name_his) = database::get_user_name_history(name.uuid.clone()) {
//                             name_history = Some(name_his);
//                         } else {
//                             println!("Couldnt get name history")
//                         }
//
//                         if let Ok(match_his) = database::get_user_match_history(name.uuid.clone()) {
//                             match_history = Some(match_his);
//                         } else {
//                             println!("Couldnt get match history")
//                         }
//
//                         if !database::user_exits(name.uuid.clone()) {
//                             if let Ok(_) = database::add_user(name.uuid.clone()) {
//                                 println!("Added new user successfully")
//                             } else {
//                                 println!("Failed to add new user")
//                             }
//                         } else {
//                             if let Ok(user) = database::get_user(name.uuid.clone()) {
//                                 times_played = user.times_played.unwrap_or(0);
//                                 last_played = user.last_played.unwrap_or(0);
//                             } else {
//                                 println!("Couldnt get user")
//                             }
//
//
//                             if let Ok(_) = database::update_user(name.uuid.clone()) {
//                                 println!("Updated new successfully")
//                             } else {
//                                 println!("Failed to update user")
//                             }
//                         }
//
//                         if !database::name_exists(name.uuid.clone(), name.game_name.clone(), name.tag_line.clone()) {
//                             if let Ok(_) = database::add_new_name(name.uuid.clone(), name.game_name.clone(), name.tag_line.clone()) {
//                                 println!("Added new name successfully")
//                             } else {
//                                 println!("Failed to add new name")
//                             }
//                         } else {
//                             println!("Still same name")
//                         }
//
//                         if !database::match_exists(name.uuid.clone(), match_id.clone()) {
//                             if let Ok(_) = database::add_new_match(name.uuid.clone(), match_id.clone()) {
//                                 println!("Added new match successfully")
//                             } else {
//                                 println!("Failed to add new match")
//                             }
//                         } else {
//                             println!("Match already exists")
//                         }
//
//
//                         let player_data = current_match.players.iter().find(|x| x.player_identity.uuid == name.uuid).unwrap();
//
//                         players.push(LoadedPlayer {
//                             uuid: name.uuid.clone(),
//                             name: name.game_name.clone(),
//                             tag: name.tag_line.clone(),
//                             team: if current_match.players.get(i).unwrap().team_id == "Blue" { TeamType::Ally } else { TeamType::Enemy },
//
//                             match_history: match_history.clone(),
//                             name_history: name_history.clone(),
//
//                             times_played,
//                             last_played,
//
//                             agent_id: player_data.agent_id.clone(),
//                             incognito: player_data.player_identity.incognito.clone(),
//                         });
//                     }
//
//                     return Ok((players, match_id.clone()))
//                 } else {
//                     // issue with retrieving names
//                     println!("couldnt get names")
//                 }
//             } else {
//                 // couldnt get pre game
//                 println!("couldnt get pre game")
//             }
//         } else {
//             // couldnt get match id
//
//             println!("couldnt get match id")
//         }
//
//         println!("unknown error");
//         return Err(Error::Unknown)
//     }
// }
