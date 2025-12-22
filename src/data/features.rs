use mongodb::{bson::doc, Client, Collection};
use anyhow::*;

use crate::models::LanguageProfile;

#[derive(Clone)]
pub struct FeaturesRepository {
    collection: Collection<LanguageProfile>
}

impl FeaturesRepository {
    pub fn new(client: &Client) -> Self {
        let col = client
            .database("linguistics")
            .collection::<LanguageProfile>("features");

        Self { collection: col }
    }

    pub async fn get(&self, lang: &str) -> Result<Option<LanguageProfile>> {
        let filter = doc! { "iso639p1": lang };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }

    pub async fn insert(&self, value: LanguageProfile) -> Result<()> {

        self.collection.insert_one(value).await?;

        Ok(())
    }
}