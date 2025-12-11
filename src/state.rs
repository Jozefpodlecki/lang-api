use anyhow::*;
use mongodb::{options::ClientOptions, Client};

use crate::{data::PronounsRepository};

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub pronouns: PronounsRepository
}

impl AppState {
    pub async fn new() -> Result<Self> {

        let connection_string = "mongodb://localhost:27017";
        let options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(options)?;

        // Ok(Self {
        //     client
        // })

        let pronouns = PronounsRepository::new(&client);

        Ok(Self {
            pronouns
        })
    }
}