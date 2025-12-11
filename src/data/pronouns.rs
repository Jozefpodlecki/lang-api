use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use anyhow::*;

use crate::models::Pronouns;

#[derive(Clone)]
pub struct PronounsRepository {
    collection: Collection<Pronouns>
}

impl PronounsRepository {
    pub fn new(client: &Client) -> Self {
        let col = client
            .database("linguistics")
            .collection::<Pronouns>("pronouns");

        Self { collection: col }
    }

    pub async fn get(&self, lang: &str) -> Result<Option<Pronouns>> {
        let filter = doc! { "language": lang };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }

    pub async fn insert(&self, value: Pronouns) -> Result<()> {

        self.collection.insert_one(value).await?;

        Ok(())
    }
}