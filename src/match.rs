use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking::Client;
use crate::{RiotAuth, LoadedPlayer, name_service, TeamType};
use crate::database;
use crate::database::{MatchHistory, NameHistory};
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
    #[serde(rename = "GamePodID")]
    pub game_pod: String,
    // #[serde(rename = "ConnectionDetails")]
    // pub connection_details: ConnectionDetails,
    #[serde(rename = "Players")]
    pub players: Vec<Player>,
}

// #[derive(serde::Deserialize, Debug, Default)]
// pub struct ConnectionDetails {
//     #[serde(rename = "GameServerHosts")]
//     pub servers: Vec<String>,
//}

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

#[derive(Debug, Default, Clone)]
pub struct MatchHandler {
    client: Client,
    pub match_id: String,
    pub game_type: String,
    pub map_path: String,
    pub game_mode: String,
    pub server: String,
    pub players: Vec<LoadedPlayer>,
}

impl MatchHandler {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            ..Default::default()
        }
    }

    pub fn get_match_id(&mut self, auth: Arc<RiotAuth>) -> Result<(), ()> {
        return match self.client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/players/{}", auth.region, auth.shard, auth.puuid))
            .bearer_auth(&auth.access_token)
            .header("X-Riot-Entitlements-JWT", &auth.token)
            .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
            .header("X-Riot-ClientVersion", &auth.client_ver)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<CurrentGamePlayer>() {
                        Ok(json) => {
                            self.match_id = json.match_id.clone();
                            Ok(())
                        },
                        Err(_) => Err(()),
                    }
                } else {
                    Err(())
                }
            },
            Err(_) => Err(()),
        }
    }

    pub fn get_match_details(&mut self, auth: Arc<RiotAuth>, latest_match_id: String) -> Result<(), ()> {
        return match self.client.get(format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}", auth.region, auth.shard, self.match_id))
            .bearer_auth(&auth.access_token)
            .header("X-Riot-Entitlements-JWT", &auth.token)
            .header("X-Riot-ClientPlatform", "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9")
            .header("X-Riot-ClientVersion", &auth.client_ver)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<CurrentGameMatch>() {
                        Ok(json) => {
                            self.match_id = json.match_id.clone();

                            if self.match_id == latest_match_id {
                                return Err(())
                            }

                            println!("passed match check");

                            //println!("{:?}", json);

                            self.map_path = json.map_id.clone();
                            self.game_mode = json.gamemode_id.clone();

                            // get server
                            let mut server = String::new();

                            let parts: Vec<&str> = json.game_pod.split('-').collect();
                            let result = parts.get(parts.len() - 2).unwrap_or(&"");

                            self.server = result.to_string();

                            let player_ids: Vec<String> = json.players.iter().map(|player| player.player_identity.uuid.clone()).collect();
                            let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

                            if let Some(player_names) = name_service::NameService::get_names(&auth, player_ids) {
                                let mut players = Vec::new();
                                let player_team = json.players.iter().find(|x| x.player_identity.uuid == auth.puuid).unwrap().team_id.clone();

                                for (i, player) in json.players.iter().enumerate() {
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
                                        times_played = user.times_played;
                                        last_played = user.last_played;
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

                                    if let Ok(_) = database::add_new_match(name.uuid.clone(), json.match_id.clone(), json.map_id.clone(), json.gamemode_id.clone(), player.agent_id.clone(), !(player.team_id == player_team), time_now) {
                                        println!("Added new match successfully")
                                    } else {
                                        println!("Failed to add new match")
                                    }

                                    let player_data = json.players.iter().find(|x| x.player_identity.uuid == name.uuid).unwrap();

                                    players.push(LoadedPlayer {
                                        uuid: name.uuid.clone(),
                                        name: name.game_name.clone(),
                                        tag: name.tag_line.clone(),
                                        team: if json.players.get(i).unwrap().team_id == "Blue" { TeamType::Ally } else { TeamType::Enemy },

                                        match_history: match_history.clone(),
                                        name_history: name_history.clone(),

                                        times_played,
                                        last_played,

                                        agent_id: player_data.agent_id.clone(),
                                        incognito: player_data.player_identity.incognito.clone(),
                                    });
                                }

                                self.players = players.clone();

                            }

                            Ok(())
                        },
                        Err(_) => Err(()),
                    }
                } else {
                    Err(())
                }
            },
            Err(_) => Err(()),
        }
    }
}