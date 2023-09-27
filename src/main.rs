use std::fs::File;
use std::io::Read;

#[derive(Debug, Default)]
struct Lockfile {
    port: String,
    password: String,
}

#[derive(Default, serde::Deserialize)]
struct Entiltement {
    #[serde(rename = "accessToken")]
    access_token: String,
    entitlements: Vec<String>,
    issuer: String,
    subject: String,
    token: String,
}

#[derive(serde::Deserialize, Debug, Default)]
struct User {
    #[serde(rename = "sub")]
    uuid: String,
}

#[derive(serde::Deserialize, Debug)]
struct GamePlayer {
    #[serde(rename = "MatchID")]
    match_id: String,
}

#[tokio::main]
async fn main() {

    let mut lockfile = Lockfile::default();
    let mut user = User::default();

    //let lockfile = std::env::var("LOCALAPPDATA");

    if let Ok(path) = std::env::var("LOCALAPPDATA") {
        let lockfile_path = format!{"{}{}", path, "\\Riot Games\\Riot Client\\Config\\lockfile"};

        let content = match std::fs::read_to_string(&lockfile_path) {
            Ok(text) => text,
            Err(_) => return,
        };

        let split_content: Vec<&str> = content.split(":").collect();

        lockfile.port = split_content.get(2).unwrap().to_string();
        lockfile.password = split_content.get(3).unwrap().to_string();
    }

    //println!("{:?}", lockfile);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // Local ip does not have ssl certificate
        .build()
        .unwrap();

    let result =  match client.get(format!("https://127.0.0.1:{}/entitlements/v1/token", lockfile.port))
        .basic_auth("riot", Some(lockfile.password))
        .send()
        .await {
            Ok(resp) => {
                //let mut text = String::new();

                if let Ok(content) = resp.json::<Entiltement>().await {
                    content
                    //text = content;
                } else {
                    return;
                }

                //text
            },
            Err(_) => return,
    };

    let player_info_res = client.get("https://auth.riotgames.com/userinfo")
        .bearer_auth(&result.access_token)
        .send()
        .await;

    let user_puuid = player_info_res.unwrap().json::<User>().await.unwrap();

    let player_match = client.get(format!("https://glz-eu-1.eu.a.pvp.net/core-game/v1/players/{}", user_puuid.uuid))
        .bearer_auth(&result.access_token)
        .header("X-Riot-Entitlements-JWT", &result.token)
        .send()
        .await;

    let game_player = player_match.unwrap().json::<GamePlayer>().await.unwrap();

    let match_data = client.get(format!("https://glz-eu-1.eu.a.pvp.net/core-game/v1/matches/{}", game_player.match_id))
        .bearer_auth(&result.access_token)
        .header("X-Riot-Entitlements-JWT", &result.token)
        .send()
        .await;

    println!("{:?}", match_data.unwrap().text().await.unwrap());
}
