use anyhow::*;

use crate::{data::*, utils::create_mongodb_client};

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub pronouns: PronounsRepository,
    pub features: FeaturesRepository,
    pub verbs: VerbsRepository,
}

impl AppState {
    pub async fn new() -> Result<Self> {

        let client = create_mongodb_client().await?;
        let pronouns = PronounsRepository::new(&client);
        let features = FeaturesRepository::new(&client);
        let verbs = VerbsRepository::new(&client);

        Ok(Self {
            pronouns,
            features,
            verbs
        })
    }
}