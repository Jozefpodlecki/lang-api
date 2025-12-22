use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPronouns {
    pub language: String,
    pub singular: SingularPronouns,
    pub plural: PluralPronouns,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SingularPronouns {
    pub first:     Option<PronounForm>,
    pub second:    Option<PronounForm>,
    pub third_m:   Option<PronounForm>,
    pub third_f:   Option<PronounForm>,
    pub third_n:   Option<PronounForm>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PluralPronouns {
    pub first:  Option<PronounForm>,
    pub second: Option<PronounForm>,
    pub third:  Option<PronounForm>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PronounForm {
    pub native: String,
    pub romanized: String,
}