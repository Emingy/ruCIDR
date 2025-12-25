use serde::Deserialize;
use std::error::Error;

const RIPE_API_URL: &str = "https://stat.ripe.net/data/country-resource-list/data.json?resource=RU";

#[derive(Debug, Deserialize)]
struct RipeResponse {
    data: RipeData,
}

#[derive(Debug, Deserialize)]
struct RipeData {
    resources: Resources,
}

#[derive(Debug, Deserialize)]
struct Resources {
    ipv4: Vec<String>,
}

pub struct RipeClient;

impl RipeClient {
    pub fn new() -> Self {
        Self
    }
    
    /// Получает список российских IPv4 адресов из RIPE
    pub async fn fetch_russian_ips(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = reqwest::get(RIPE_API_URL).await?;
        let ripe_data: RipeResponse = response.json().await?;
        Ok(ripe_data.data.resources.ipv4)
    }
}