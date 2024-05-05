use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{RiotAuth, LoadedPlayer, name_service, TeamType};
use crate::database;
use crate::database::{MatchHistory, NameHistory};

#[derive(Debug)]
pub struct MatchHandler {
    pub match_id: String,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct CurrentGamePlayer {
    #[serde(rename = "MatchID")]
    pub match_id: String,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct CurrentGameMatch {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "MapID")]
    pub map_id: String,
    #[serde(rename = "ModeID")]
    pub gamemode_id: String,
    #[serde(rename = "Players")]
    pub players: Vec<Player>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Player {
    #[serde(rename = "TeamID")]
    pub team_id: String,
    #[serde(rename = "CharacterID")]
    pub agent_id: String,
    #[serde(rename = "PlayerIdentity")]
    pub player_identity: PlayerIdentity,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct PlayerIdentity {
    #[serde(rename = "Subject")]
    pub uuid: String,
    #[serde(rename = "PlayerCardID")]
    pub card_id: String,
    #[serde(rename = "PlayerTitleID")]
    pub title_id: String,
    #[serde(rename = "AccountLevel")]
    pub level: u16,
    #[serde(rename = "Incognito")]
    pub incognito: bool,
    #[serde(rename = "HideAccountLevel")]
    pub hide_level: bool,
}

#[derive(Debug)]
pub enum Error { NotPreGame, ClientError, RiotError, Unknown }

impl CurrentGamePlayer {
    pub fn get_match_id(auth: &RiotAuth) -> Option<String> {
        let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build() {
            Ok(client) => client,
            Err(_) => return None,
        };

        return match client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/players/{}", auth.region, auth.shard, auth.puuid))
            .bearer_auth(&auth.access_token)
            .header("X-Riot-Entitlements-JWT", &auth.token)
            .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
            .header("X-Riot-ClientVersion", &auth.client_ver)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<CurrentGamePlayer>() {
                        Ok(json) => Some(json.match_id.clone()),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            },
            Err(_) => None,
        }
    }

    pub fn get_match(auth: &RiotAuth, match_id: String) -> Option<CurrentGameMatch> {
        let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build() {
            Ok(client) => client,
            Err(_) => return None,
        };

        return match client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}", auth.region, auth.shard, match_id))
            .bearer_auth(&auth.access_token)
            .header("X-Riot-Entitlements-JWT", &auth.token)
            .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
            .header("X-Riot-ClientVersion", &auth.client_ver)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<CurrentGameMatch>() {
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

    pub fn get_players(auth: Arc<RiotAuth>, prev_match_id: String) -> Result<(Vec<LoadedPlayer>, String), Error> {
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        if let Some(match_id) = CurrentGamePlayer::get_match_id(&auth) {

            if prev_match_id == match_id {
                println!("same match id");
                return Err(Error::NotPreGame)
            }

            if let Some(current_match) = CurrentGamePlayer::get_match(&auth, match_id.clone()) {
                let player_ids: Vec<String> = current_match.players.iter().map(|player| player.player_identity.uuid.clone()).collect();

                if let Some(player_names) = name_service::NameService::get_names(&auth, player_ids) {
                    let mut players = Vec::new();
                    let player_team = current_match.players.iter().find(|x| x.player_identity.uuid == auth.puuid).unwrap().team_id.clone();

                    for (i, player) in current_match.players.iter().enumerate() {
                        let name = player_names.iter().find(|x| x.uuid == player.player_identity.uuid).unwrap();

                        let mut times_played: i64 = 0;
                        let mut last_played: i64 = time_now;
                        let mut match_history: Vec<MatchHistory> = Vec::new();
                        let mut name_history: Vec<NameHistory> = Vec::new();

                        if let Ok(name_his) = database::get_user_name_history(name.uuid.clone()) {
                            name_history = name_his;
                        } else {
                            println!("Couldnt get name history")
                        }

                        if let Ok(match_his) = database::get_user_match_history(name.uuid.clone()) {
                            match_history = match_his;
                        } else {
                            println!("Couldnt get match history")
                        }

                        if let Ok(user) = database::get_user(name.uuid.clone()) {
                            times_played = user.times_played.unwrap_or(0);
                            last_played = user.last_played.unwrap_or(0);
                        } else {
                            println!("Couldnt get user")
                        }

                        if let Ok(_) = database::update_user(name.uuid.clone()) {
                            println!("Updated new successfully")
                        } else {
                            println!("Unable to update user")
                        }

                        if let Ok(_) = database::add_new_name(name.uuid.clone(), name.game_name.clone(), name.tag_line.clone()) {
                            println!("Added new name successfully")
                        } else {
                            println!("Failed to add new name")
                        }

                        if let Ok(_) = database::add_new_match(name.uuid.clone(), current_match.match_id.clone(), current_match.map_id.clone(), current_match.gamemode_id.clone(), player.agent_id.clone(), !(player.team_id == player_team)) {
                            println!("Added new match successfully")
                        } else {
                            println!("Failed to add new match")
                            }

                        let player_data = current_match.players.iter().find(|x| x.player_identity.uuid == name.uuid).unwrap();

                        players.push(LoadedPlayer {
                            uuid: name.uuid.clone(),
                            name: name.game_name.clone(),
                            tag: name.tag_line.clone(),
                            team: if current_match.players.get(i).unwrap().team_id == "Blue" { TeamType::Ally } else { TeamType::Enemy },

                            match_history: match_history.clone(),
                            name_history: name_history.clone(),

                            times_played,
                            last_played,

                            agent_id: player_data.agent_id.clone(),
                            incognito: player_data.player_identity.incognito.clone(),
                        });
                    }

                    return Ok((players, match_id.clone()))
                } else {
                    // issue with retrieving names
                    println!("couldnt get names")
                }
            } else {
                // couldnt get pre game
                println!("couldnt get pre game")
            }
        } else {
            // couldnt get match id

            println!("couldnt get match id")
        }

        println!("unknown error");
        return Err(Error::Unknown)
    }
}
