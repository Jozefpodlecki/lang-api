use futures::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use anyhow::*;

use crate::models::{Pronouns, Verb};

#[derive(Clone)]
pub struct VerbsRepository {
    collection: Collection<Verb>
}

impl VerbsRepository {
    pub fn new(client: &Client) -> Self {
        let col = client
            .database("linguistics")
            .collection::<Verb>("verbs");

        Self { collection: col }
    }

    pub async fn get_all(&self, lang: &str) -> Result<Vec<Verb>> {
        let filter = doc! { "language": lang };
        let cursor = self.collection.find(filter).await?;
        let result: Vec<Verb> = cursor.try_collect().await?;
        Ok(result)
    }

    pub async fn get(&self, lang: &str) -> Result<Option<Verb>> {
        let filter = doc! { "language": lang };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }

    pub async fn insert(&self, value: Verb) -> Result<()> {

        self.collection.insert_one(value).await?;

        Ok(())
    }
}