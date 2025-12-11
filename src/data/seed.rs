use std::collections::HashMap;

use mongodb::{options::ClientOptions, Client};
use anyhow::Result;

use crate::{data::pronouns::PronounsRepository, models::{PluralPronouns, Pronouns, SingularPronouns}};

pub async fn seed_data() -> Result<()> {

    let connection_string = "mongodb://localhost:27017";
    let options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(options)?;

    let database = client.database("linguistics");

    // client.database("linguistics").drop().await?;

    let collections = database.list_collection_names().await?;
    
    if collections.contains(&"pronouns".to_string()) {
        println!("Seed database already initialized. Skipping seeding.");
        return Ok(());
    }

    seed_pronouns(&client).await?;

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