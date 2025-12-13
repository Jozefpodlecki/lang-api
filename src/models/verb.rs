use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Verb {
    pub language: String,
    pub lemma: String,
    pub particles: Vec<String>,

    // Classification
    pub transitivity: Option<String>,
    pub auxiliary: Option<String>,
    pub irregular: bool,

    // Semantics
    pub gloss: Option<String>,
    pub tags: Vec<String>,

    // Conjugation tables
    pub conjugations: HashMap<String, ConjugationTable>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConjugationTable {
    pub tense: String,
    pub forms: HashMap<String, String>,
}
