use reqwest::Client;
use anyhow::Result;
use crate::models::LanguageProfile;

pub struct LanguageApi {
    base_url: String,
    client: Client,
}

impl LanguageApi {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn get_language_metadata(&self, iso639p1: &str) -> Result<LanguageProfile> {
        let url = format!("{}/api/v1/language/{}", self.base_url, iso639p1);
        
        let response = self.client.get(url).send().await?;
        let data = response.json().await?;
        
        Ok(data)
    }
}