use reqwest::blocking::Client;
#[derive(serde::Deserialize, Debug, Default, Clone)]
pub struct AgentDetail {
    pub data: Vec<AgentDetailData>,
}

#[derive(serde::Deserialize, Debug, Default, Clone)]
pub struct AgentDetailData {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "displayName")]
    pub name: String,
    #[serde(rename = "displayIconSmall")]
    pub icon: String
}

#[derive(serde::Deserialize, Debug, Default, Clone)]
pub struct MapDetail {
    pub data: Vec<MapDetailData>,
}

#[derive(serde::Deserialize, Debug, Default, Clone)]
pub struct MapDetailData {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "displayName")]
    pub name: String,
    #[serde(rename = "splash")]
    pub icon: String,
    #[serde(rename = "mapUrl")]
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    client: Client,
    pub agents: Vec<AgentDetailData>,
    pub maps: Vec<MapDetailData>,
}

impl ImageData {
    // TODO: Save images locally
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            agents: Vec::new(),
            maps: Vec::new(),
        }
    }

    pub fn get_agents(&mut self) -> Result<(), ()> {
        return match self.client.get("https://valorant-api.com/v1/agents?isPlayableCharacter=true")
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<AgentDetail>() {
                        Ok(json) => {
                            self.agents = json.data.clone();
                            Ok(())
                        },
                        Err(_) => Err(())
                    }
                } else {
                    Err(())
                }
            },
            Err(_) => return Err(()),
        }
    }

    pub fn get_maps(&mut self) -> Result<(), ()> {
        return match self.client.get("https://valorant-api.com/v1/maps")
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<MapDetail>() {
                        Ok(json) => {
                            self.maps = json.data.clone();
                            Ok(())
                        },
                        Err(_) => Err(())
                    }
                } else {
                    Err(())
                }
            },
            Err(_) => return Err(()),
        }
    }
}

