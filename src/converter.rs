use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::Value;
use crate::database;

#[derive(serde::Deserialize, Debug, Default, Clone)]
pub struct VryPlayerHistory {
    name: String,
    agent: String,
    map: String,
    match_id: String,
    #[serde(rename = "epoch")]
    time: i64,
}

pub struct Converter {}

impl Converter {
    pub fn get_file(path: &Path) -> String{
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        contents
    }

    pub fn convert_vry_history(data: String) -> (i32, i32, i32){
        let v: Value = serde_json::from_str(&*data).unwrap();
        let (mut success, mut fail, mut total) = (0, 0, 0);

        if let Some(obj) = v.as_object() {
            for (uuid, player_data) in obj.iter() {
                let mut total_matches: i64 = 0;
                let mut last_played: i64 = 0;
                let mut names: Vec<String> = Vec::new();

                if let Some(data_array) = player_data.as_array() {
                    total_matches = data_array.len() as i64;

                    for history in data_array {
                        total += 1;
                        last_played = history.get("epoch").unwrap().as_f64().unwrap() as i64;
                        names.push(history.get("name").unwrap().to_string().trim_matches('\"').to_string());

                        match database::add_new_match(
                            uuid.clone(),
                            history.get("match_id").unwrap().to_string(),
                             if let Some(map_obj) = history.get("map").unwrap().as_object() { map_obj.get("name").unwrap().to_string().trim_matches('\"').to_string() } else { history.get("map").unwrap().to_string().trim_matches('\"').to_string() },
                            "".to_string(),
                            history.get("agent").unwrap().to_string().trim_matches('\"').to_string(),
                            false,
                            last_played,
                        ) {
                            Ok(_) => success += 1,
                            Err(_) => fail += 1,
                        }
                    }
                }

                // Filter duplicate names
                let set: HashSet<_> = names.drain(..).collect();
                let filtered_names: Vec<String> = set.into_iter().collect();

                for name in filtered_names {
                    let mut name_split = name.split("#");

                    match database::add_new_name(
                        uuid.clone(),
                        name_split.next().unwrap().to_string(),
                        name_split.next().unwrap().to_string(),
                    ) {
                        Ok(_) => println!("added new name successfully"),
                        Err(_) => println!("couldnt add new name"),
                    }
                }

                match database::add_user_full(uuid.to_owned(), total_matches, last_played) {
                    Ok(_) => println!("Added new user"),
                    Err(_) => println!("Failed to add new full user"),
                }
            }
        }

        (success, fail, total)
    }
}

