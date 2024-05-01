use crate::loader;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Authorization {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "token")]
    pub token: String,
}

#[derive(Debug)]
pub enum Response { ValorantNotOpen, ClientNotBuilt, SerdeError }

pub fn get_auth(port: String, password: String) -> Result<(String, String), Response> {
    let client = match reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(client) => client,
            Err(_) => return Err(Response::ClientNotBuilt),
    };

    println!("Client was built");

    let res = match client.get(format!("https://127.0.0.1:{}/entitlements/v1/token", port)).basic_auth("riot", Some(password)).send() {
        Ok(response) => {
            println!("{:?}", response);
            response
        },
        Err(_) => return Err(Response::ValorantNotOpen),
    };

    println!("Got response");

    
    let auth = match res.json::<Authorization>() {
        Ok(auth) => auth,
        Err(err) => {
            println!("{:?}", err);
            return Err(Response::SerdeError)
        },
    };

    return Ok((auth.token.clone(), auth.access_token.clone()));
}