use std::collections::HashMap;

use anyhow::Result;
use mongodb::Client;

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

    Ok(())
}

pub async fn seed_features(client: &Client) -> Result<()> {
    let features = FeaturesRepository::new(client);

      let en = LanguageFeatures {
        language: "en".into(),

        // Syntax
        word_order: Some("SVO".into()),
        topic_prominent: Some(false),

        // Morphology
        morphological_typology: Some("analytic".into()),
        has_inflection: Some(false),        // only minor residual inflection
        has_conjugation: Some(true),
        has_tense_marking: Some(true),

        // Nominal system
        has_grammatical_gender: Some(false),
        gender_count: Some(0),
        has_cases: Some(false),
        case_count: Some(0),
        classifier_system: Some(false),
        classifier_count: Some(0),

        // Pronouns & pragmatics
        pro_drop: Some(false),
        politeness_levels: Some(false),

        // Aspect & particles
        aspect_prominent: Some(false),       // has aspect, but not typologically dominant
        extensive_particles: Some(false),

        // Writing system
        writing_system: Some("Latin".into()),
        script_direction: Some("LTR".into()),

        // Phonology
        tonal_language: Some(false),
        tone_count: Some(0),
    };

    features.insert(en).await?;

     let de = LanguageFeatures {
        language: "de".into(),

        // Syntax
        word_order: Some("SVO/SOV".into()),   // German is underlyingly SOV in embedded clauses
        topic_prominent: Some(false),

        // Morphology
        morphological_typology: Some("fusional".into()),
        has_inflection: Some(true),
        has_conjugation: Some(true),
        has_tense_marking: Some(true),

        // Nominal system
        has_grammatical_gender: Some(true),
        gender_count: Some(3),
        has_cases: Some(true),
        case_count: Some(4),
        classifier_system: Some(false),
        classifier_count: Some(0),

        // Pronouns & pragmatics
        pro_drop: Some(false),
        politeness_levels: Some(true),       // "Sie" vs "du"

        // Aspect & particles
        aspect_prominent: Some(false),        // German expresses aspect lexically
        extensive_particles: Some(false),     // separable prefixes exist but not particles like Japanese

        // Writing system
        writing_system: Some("Latin".into()),
        script_direction: Some("LTR".into()),

        // Phonology
        tonal_language: Some(false),
        tone_count: Some(0),
    };

    features.insert(de).await?;

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