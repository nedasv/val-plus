#[derive(Debug, Default, serde::Deserialize)]
pub struct Authorization {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "token")]
    pub token: String,
}

pub fn get_auth(port: String, password: String) -> Option<(String, String)> {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return None,
    };

    return match client.get(format!("https://127.0.0.1:{}/entitlements/v1/token", port)).basic_auth("riot", Some(password)).send() {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<Authorization>() {
                    Ok(auth) => {
                        Some((auth.token.clone(), auth.access_token.clone()))
                    },
                    Err(_) => None,
                }
            } else {
                None
            }
        },
        Err(_) => None,
    }
}