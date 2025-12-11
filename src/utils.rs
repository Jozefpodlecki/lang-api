use mongodb::{options::ClientOptions, Client};
use anyhow::Result;

pub async fn create_mongodb_client() -> Result<Client> {
    let connection_string = "mongodb://localhost:27017";
    let options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(options)?;

    Ok(client)
}