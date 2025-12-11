use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pronouns {
    pub language: String,
    pub singular: SingularPronouns,
    pub plural: PluralPronouns,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingularPronouns {
    pub first:     Option<String>,
    pub second:    Option<String>,
    pub third_m:   Option<String>,
    pub third_f:   Option<String>,
    pub third_n:   Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluralPronouns {
    pub first:  Option<String>,
    pub second: Option<String>,
    pub third:  Option<String>,
}