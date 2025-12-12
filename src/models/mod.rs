use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod language;
mod verb;

pub use language::*;
pub use verb::*;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Pronouns {
    pub language: String,
    pub singular: SingularPronouns,
    pub plural: PluralPronouns,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SingularPronouns {
    pub first:     Option<String>,
    pub second:    Option<String>,
    pub third_m:   Option<String>,
    pub third_f:   Option<String>,
    pub third_n:   Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PluralPronouns {
    pub first:  Option<String>,
    pub second: Option<String>,
    pub third:  Option<String>,
}