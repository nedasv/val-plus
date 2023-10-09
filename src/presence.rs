use reqwest::blocking::Client;

use crate::loader::Lockfile;

pub struct Presence {}

impl Presence {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load(&self, lockfile: &Lockfile) -> Result<(), ()> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let req = match client.get(format!("https://127.0.0.1:{}/chat/v4/presences", lockfile.port))
            .basic_auth("riot", Some(lockfile.password.clone()))
            .send()
        {
            Ok(req) => {
                println!("PRESENCE");
                println!("{:?}", req.text().unwrap());
            },
            Err(err) => {
                println!("{:?}", err);
                return Err(());
            }
        };

        return Ok(())
    }
}