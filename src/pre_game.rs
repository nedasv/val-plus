use crate::{LoadedPlayer, name_service, pre_game, RiotAuth, TeamType};

#[derive(serde::Deserialize, Debug, Default)]
pub struct PreGameId {
    #[serde(rename = "MatchID")]
    pub match_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct PreGame {
    #[serde(rename = "AllyTeam")]
    pub ally_team: Team,
}

#[derive(serde::Deserialize, Debug)]
pub struct Team {
    #[serde(rename = "Players")]
     pub players: Vec<Player>
}

#[derive(serde::Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "Subject")]
    pub uuid: String,
    #[serde(rename = "CharacterID")]
    pub agent_id: String,
    #[serde(rename = "CharacterSelectionState")]
    pub selection_state: String,
    #[serde(rename = "CompetitiveTier")]
    pub rank: u8,
    #[serde(rename = "PlayerIdentity")]
    pub player_identity: PlayerIdentity
}

#[derive(serde::Deserialize, Debug)]
pub struct PlayerIdentity {
    #[serde(rename = "AccountLevel")]
    pub level: u16,
    #[serde(rename = "Incognito")]
    pub incognito: bool,
    #[serde(rename = "HideAccountLevel")]
    pub hide_level: bool,
}

#[derive(Debug)]
pub enum Error { NotPreGame, ClientError, RiotError, Unknown }

impl PreGame {
    pub fn get_match_id(auth: &RiotAuth) -> Result<String, Error> {
        let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build() {
                Ok(client) => client,
                Err(_) => return Err(Error::ClientError),
            };

        let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/pregame/v1/players/{}", auth.region, auth.shard, auth.puuid))
            .bearer_auth(&auth.access_token)
            .header("X-Riot-Entitlements-JWT", &auth.token)
            .send() {
                Ok(resp) => resp,
                Err(err) => {
                    println!("{:?}", err);
                    return Err(Error::RiotError)
                }
            };



        if resp.status().is_success() {
            return if let Ok(pre_game_id) = resp.json::<PreGameId>() {
                Ok(pre_game_id.match_id.clone())
            } else {
                Err(Error::NotPreGame)
            }
        }

        return Err(Error::Unknown)
    }

    pub fn get_pre_game(auth: &RiotAuth, match_id: String) -> Result<PreGame, Error>{
        let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build() {
            Ok(client) => client,
            Err(_) => return Err(Error::ClientError),
        };

        let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/pregame/v1/matches/{}", auth.region, auth.shard, &match_id))
            .bearer_auth(&auth.access_token)
            .header("X-Riot-Entitlements-JWT", &auth.token)
            .send() {
            Ok(resp) => resp,
            Err(err) => {
                println!("{:?}", err);
                return Err(Error::RiotError)
            }
        };

        return Ok(resp.json::<PreGame>().unwrap())
    }

    pub fn get_players(auth: &RiotAuth, prev_match_id: String) -> Result<Vec<LoadedPlayer>, Error> {
        if let Ok(match_id) = self::PreGame::get_match_id(auth) {

            if prev_match_id == match_id {
                return Err(Error::NotPreGame)
            }

            if let Ok(pre_game) = self::PreGame::get_pre_game(auth, match_id) {
                let player_ids: Vec<String> = pre_game.ally_team.players.iter().map(|player| player.uuid.clone()).collect();

                if let Some(player_names) = name_service::NameService::get_names(auth, player_ids) {
                    let mut players = Vec::new();

                    for name in player_names {
                        players.push(LoadedPlayer {
                            uuid: name.uuid,
                            name: name.game_name,
                            tag: name.tag_line,
                            team: TeamType::Ally,

                            match_history: Vec::new(),
                            name_history: Vec::new(),

                            last_played: 0_i64,
                            times_played: 0_i64,

                            agent_id: String::from("0"),
                            incognito: false,
                        });
                    }

                    return Ok(players)
                } else {
                    // issue with retrieving names
                }
            } else {
                // couldnt get pre game
            }
        } else {
            // couldnt get match id
        }

        return Err(Error::Unknown)
    }
}



