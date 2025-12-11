use mongodb::{bson::doc, Client, Collection};
use anyhow::*;

use crate::models::{LanguageFeatures};

#[derive(Clone)]
pub struct FeaturesRepository {
    collection: Collection<LanguageFeatures>
}

impl FeaturesRepository {
    pub fn new(client: &Client) -> Self {
        let col = client
            .database("linguistics")
            .collection::<LanguageFeatures>("features");

        Self { collection: col }
    }

    pub async fn get(&self, lang: &str) -> Result<Option<LanguageFeatures>> {
        let filter = doc! { "language": lang };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }

    pub async fn insert(&self, value: LanguageFeatures) -> Result<()> {

        self.collection.insert_one(value).await?;

        Ok(())
    }
}