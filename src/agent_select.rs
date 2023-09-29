use crate::{loader::User, auth::Authorization};

#[derive(serde::Deserialize, Debug, Default)]
pub struct PreGameId {
    #[serde(rename = "MatchID")]
    pub match_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct PreGame {
    #[serde(rename = "AllyTeam")]
    ally_team: Team,
}

#[derive(serde::Deserialize, Debug)]
pub struct Team {
    #[serde(rename = "Players")]
     players: Vec<Player>
}

#[derive(serde::Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "Subject")]
    uuid: String,
    #[serde(rename = "CharacterID")]
    agent_id: String,
    #[serde(rename = "CharacterSelectionState")]
    selection_state: String,
    #[serde(rename = "CompetitiveTier")]
    rank: u8,
    #[serde(rename = "PlayerIdentity")]
    player_identity: PlayerIdentity
}

#[derive(serde::Deserialize, Debug)]
pub struct PlayerIdentity {
    #[serde(rename = "PlayerCardID")]
    card_id: String,
    #[serde(rename = "PlayerTitleID")]
    title_id: String,
    #[serde(rename = "AccountLevel")]
    level: u16,
    #[serde(rename = "Incognito")]
    incognito: bool,
    #[serde(rename = "HideAccountLevel")]
    hide_level: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct SeasonInfo {
    #[serde(rename = "NumberOfWins")]
    wins: u16,
    #[serde(rename = "Rank")]
    rank: u16,
    #[serde(rename = "LeaderboardRank")]
    leaderboard_rank: u16,
}

#[derive(Debug)]
pub enum Error { NotPreGame, ClientError, RiotError, Unknown }

impl PreGameId {
    pub fn get_match_id(&self, user: &User, auth: &Authorization) -> Result<PreGameId, Error> {
        let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build() {
                Ok(client) => client,
                Err(_) => return Err(Error::ClientError),
            };

        let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/pregame/v1/players/{}", &user.region, &user.shard, &user.puuid))
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
            if let Ok(pre_game) = resp.json::<PreGameId>() {
                return Ok(pre_game);
            } else {
                return Err(Error::NotPreGame);
            }
        }

        return Err(Error::Unknown);
    }
}

pub fn get_pre_game(auth: &Authorization, user: &User, match_id: &PreGameId) -> Result<(), Error>{
    let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build() {
                Ok(client) => client,
                Err(_) => return Err(Error::ClientError),
            };

    let resp = match client.get(format!("https://glz-{}-1.{}.a.pvp.net/pregame/v1/matches/{}", &user.region, &user.shard, &match_id.match_id))
        .bearer_auth(&auth.access_token)
        .header("X-Riot-Entitlements-JWT", &auth.token)
        .send() {
            Ok(resp) => resp,
            Err(err) => {
                println!("{:?}", err);
                return Err(Error::RiotError)
            }
        };

    println!("{:?}", resp.json::<PreGame>().unwrap());

    return Ok(())
}

