use std::{fs, path::Path};

use anyhow::Result;
use log::*;
use mongodb::Client;
use serde::de::DeserializeOwned;

use crate::{data::{pronouns::PronounsRepository, FeaturesRepository, VerbsRepository}, models::*, utils::create_mongodb_client};

pub async fn seed_data() -> Result<()> {

    let client = create_mongodb_client().await?;

    let database = client.database("linguistics");

    client.database("linguistics").drop().await?;

    let collections = database.list_collection_names().await?;
    
    if collections.contains(&"pronouns".to_string()) {
        println!("Seed database already initialized. Skipping seeding.");
        return Ok(());
    }

    seed_language_metadata(&client).await?;
    seed_pronouns(&client).await?;
    seed_verbs(&client).await?;

    Ok(())
}

pub fn load_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let data = fs::read(path)?;
    let parsed = serde_json::from_slice(&data)?;
    Ok(parsed)
}

pub async fn seed_language_metadata(client: &Client) -> Result<()> {
    let features = FeaturesRepository::new(client);

    for entry in fs::read_dir("data/language-families")? {
        let entry = entry?;
        let path = entry.path();

        debug!("loading {}", path.file_name().unwrap().display());

        let documents: Vec<LanguageFeatures>  = load_json(&path)?;

        for document in documents {
            features.insert(document).await?;
        }
    }
    
    debug!("loaded language metadata");

    Ok(())
}

pub async fn seed_verbs(client: &Client) -> Result<()> {
    let verbs = VerbsRepository::new(client);

    let documents: Vec<Verb> = load_json("data/en/verbs.json")?;

    for document in documents {
        // debug!("loading verb: {}", document.lemma);
        verbs.insert(document).await?;
    }

    let documents: Vec<Verb> = load_json("data/de/verbs.json")?;

    for document in documents {
        verbs.insert(document).await?;
    }

    debug!("loaded verbs");

    Ok(())
}

pub async fn seed_pronouns(client: &Client) -> Result<()> {
    let pronouns = PronounsRepository::new(client);

    let documents: Vec<PersonalPronouns> = load_json("data/personal-pronouns.json")?;

    for document in documents {
        pronouns.insert(document).await?;
    }

    debug!("loaded pronouns");

    Ok(())
}