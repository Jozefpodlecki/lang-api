use mongodb::{bson::doc, Client, Collection};
use anyhow::*;

use crate::models::PersonalPronouns;

#[derive(Clone)]
pub struct PronounsRepository {
    collection: Collection<PersonalPronouns>
}

impl PronounsRepository {
    pub fn new(client: &Client) -> Self {
        let col = client
            .database("linguistics")
            .collection::<PersonalPronouns>("pronouns");

        Self { collection: col }
    }

    pub async fn get(&self, lang: &str) -> Result<Option<PersonalPronouns>> {
        let filter = doc! { "language": lang };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }

    pub async fn insert(&self, value: PersonalPronouns) -> Result<()> {

        self.collection.insert_one(value).await?;

        Ok(())
    }
}