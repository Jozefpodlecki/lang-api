use std::{collections::HashMap, fs};

use anyhow::Result;
use log::*;
use mongodb::Client;
use serde::de::DeserializeOwned;

use crate::{data::{pronouns::PronounsRepository, FeaturesRepository}, models::{LanguageFeatures, PluralPronouns, Pronouns, SingularPronouns}, utils::create_mongodb_client};

pub async fn seed_data() -> Result<()> {

    let client = create_mongodb_client().await?;

    let database = client.database("linguistics");

    client.database("linguistics").drop().await?;

    let collections = database.list_collection_names().await?;
    
    if collections.contains(&"pronouns".to_string()) {
        println!("Seed database already initialized. Skipping seeding.");
        return Ok(());
    }

    seed_features(&client).await?;
    seed_pronouns(&client).await?;
    seed_verbs(&client).await?;

    Ok(())
}

pub fn load_json<T: DeserializeOwned>(path: &str) -> Result<T> {
    let data = fs::read(path)?;
    let parsed = serde_json::from_slice(&data)?;
    Ok(parsed)
}

pub async fn seed_features(client: &Client) -> Result<()> {
    let features = FeaturesRepository::new(client);

    let documents: Vec<LanguageFeatures>  = load_json("data/features.json")?;

    for document in documents {
        features.insert(document).await?;
    }
    
    debug!("loaded languages");

    Ok(())
}

pub async fn seed_verbs(client: &Client) -> Result<()> {
    let verbs = FeaturesRepository::new(client);

    let documents: Vec<LanguageFeatures>  = load_json("data/verbs.json")?;

    for document in documents {
        verbs.insert(document).await?;
    }

    Ok(())
}

pub async fn seed_pronouns(client: &Client) -> Result<()> {
    let pronouns = PronounsRepository::new(client);

    let document = Pronouns {
        language: "en".into(),
        singular: SingularPronouns {
            first:   Some("I".into()),
            second:  Some("you".into()),
            third_m: Some("he".into()),
            third_f: Some("she".into()),
            third_n: Some("it".into()),
        },
        plural: PluralPronouns {
            first:  Some("we".into()),
            second: Some("you".into()),
            third:  Some("they".into()),
        },
    };

    pronouns.insert(document).await?;

    let document = Pronouns {
        language: "de".into(),
        singular: SingularPronouns {
            first:   Some("ich".into()),
            second:  Some("du".into()),
            third_m: Some("er".into()),
            third_f: Some("sie".into()),
            third_n: Some("es".into()),
        },
        plural: PluralPronouns {
            first:  Some("wir".into()),
            second: Some("ihr".into()),
            third:  Some("sie".into()),
        },
    };

    pronouns.insert(document).await?;


    Ok(())
}