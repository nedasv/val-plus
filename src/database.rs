use std::time::{SystemTime, UNIX_EPOCH};
use turbosql::{execute, select, Turbosql, update};

#[derive(Turbosql, Default, Debug, Clone)]
pub struct UserDatabase {
    pub rowid: Option<i64>,
    pub uuid: String,
    pub times_played: i64,
    pub last_played: i64,
}

#[derive(Turbosql, Default, Debug, Clone)]
pub struct MatchHistory {
    pub rowid: Option<i64>,
    pub uuid: String,
    pub match_id: String,
    pub map_id: String,
    pub gamemode_id: Option<String>,
    pub enemy: Option<bool>,
    pub agent_id: String,
    pub match_time: i64,
}

#[derive(Turbosql, Default, Debug, Clone)]
pub struct NameHistory {
    pub rowid: Option<i64>,
    pub uuid: String,
    pub name: String,
    pub tag: String,
    pub name_time: Option<i64>,
}

fn user_exits(uuid: &String) -> bool {
    if select!(UserDatabase "WHERE uuid =" uuid).is_ok() {
        return true
    }
    return false
}
fn add_user(uuid: &String) -> Result<(), ()> {
    let res = UserDatabase {
        uuid: uuid.clone(),
        times_played: 0,
        last_played: 0,
        ..Default::default()
    }.insert();

    if res.is_ok() {
        return Ok(())
    }
    Err(())
}

pub fn get_user(uuid: String) -> Result<UserDatabase, ()> {
    if let Ok(user) = select!(UserDatabase "WHERE uuid =" uuid) {
        return Ok(user)
    }

    Err(())
}

pub fn update_user(uuid: String) -> Result<(), ()> {
    if !user_exits(&uuid) {
        if let Err(_) = add_user(&uuid) {
            return Err(())
        }
    }

    if let Ok(a) = execute!("UPDATE userdatabase SET times_played = times_played + 1, last_played =" SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() "WHERE uuid=" uuid) {
        return Ok(())
    }

    Err(())
}

pub fn add_user_full(uuid: String, times_played: i64, last_played: i64) -> Result<(), ()> {
    let res = UserDatabase {
        uuid: uuid.clone(),
        times_played,
        last_played,
        ..Default::default()
    }.insert();

    if res.is_ok() {
        return Ok(())
    }
    Err(())
}

pub fn get_user_name_history(uuid: String) -> Result<Vec<NameHistory>, ()>{
    if select!(NameHistory "WHERE uuid=" uuid).is_ok() {
        if let Ok(history) = select!(Vec<NameHistory> "WHERE uuid=" uuid) {
            return Ok(history);
        }
    }

    Err(())
}

pub fn add_new_name(uuid: String, name: String, tag: String) -> Result<(), ()> {
    if !name_exists(&uuid, &name, &tag) {
        let res = NameHistory {
            uuid,
            name,
            tag,
            name_time: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64),
            ..Default::default()
        }.insert();

        if res.is_ok() {
            return Ok(())
        }
    }

    Err(())
}

fn name_exists(uuid: &String, name: &String, tag: &String) -> bool {
    if select!(NameHistory "WHERE uuid=" uuid "AND name=" name "AND tag=" tag).is_ok() {
        return true
    }
    return false
}


pub fn get_user_match_history(uuid: String) -> Result<Vec<MatchHistory>, ()>{
    if select!(MatchHistory "WHERE uuid=" uuid).is_ok() {
        if let Ok(history) = select!(Vec<MatchHistory> "WHERE uuid=" uuid) {
            return Ok(history);
        }
    }

    Err(())
}

pub fn add_new_match(uuid: String, match_id: String, map_id: String, gamemode_id: String, agent_id: String, enemy: bool, time_played: i64) -> Result<(), ()> {
    if !match_exists(&uuid, &match_id) {
        let res = MatchHistory {
            uuid,
            match_id,
            match_time: time_played,
            map_id,
            gamemode_id: Some(gamemode_id),
            agent_id,
            enemy: Some(enemy),
            ..Default::default()
        }.insert();

        if res.is_ok() {
            return Ok(())
        } else {
            println!("{:?}", res);
        }
    }

    Err(())
}

fn match_exists(uuid: &String, match_id: &String) -> bool {
    if select!(MatchHistory "WHERE uuid=" uuid "AND match_id=" match_id).is_ok() {
        return true
    }
    return false
}

