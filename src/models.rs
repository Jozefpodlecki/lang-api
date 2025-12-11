use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LanguageFeatures {
    pub language: String,

    // Syntax
    pub word_order: Option<String>,              // SVO, SOV, etc.
    pub topic_prominent: Option<bool>,           // Chinese, Japanese

    // Morphology
    pub morphological_typology: Option<String>,  // analytic, agglutinative, fusional, templatic
    pub has_inflection: Option<bool>,            // Japanese: yes; Chinese: no
    pub has_conjugation: Option<bool>,
    pub has_tense_marking: Option<bool>,         // Japanese yes; Chinese no (aspect instead)

    // Nominal system
    pub has_grammatical_gender: Option<bool>,
    pub gender_count: Option<u8>,
    pub has_cases: Option<bool>,
    pub case_count: Option<u8>,
    pub classifier_system: Option<bool>,         // Chinese, Japanese: yes
    pub classifier_count: Option<u16>,           // Optional, but useful (Chinese ~150)

    // Pronominal system
    pub pro_drop: Option<bool>,                  // Chinese, Japanese: yes
    pub politeness_levels: Option<bool>,         // Japanese: yes; Chinese: mild

    // Aspect and particles
    pub aspect_prominent: Option<bool>,          // Chinese: strong
    pub extensive_particles: Option<bool>,       // Japanese: strong; Chinese: moderate

    // Writing system
    pub writing_system: Option<String>,          // "Chinese characters", "Kana + Kanji"
    pub script_direction: Option<String>,        // "LTR", "RTL", "vertical"

    // Phonology
    pub tonal_language: Option<bool>,            // Chinese: yes; Japanese: pitch-accent
    pub tone_count: Option<u8>,                  // Mandarin: 4 or 5 depending on analysis
}

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