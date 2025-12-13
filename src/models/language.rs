use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// High-level typological and structural features of a natural language.
///
/// This model is intentionally descriptive (not prescriptive) and is designed
/// to support all known human languages without schema changes.
///
/// All subcomponents are optional to allow partial or evolving descriptions.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LanguageFeatures {
    /// ISO 639-1 or ISO 639-3 language identifier (e.g. "en", "de", "zh")
    pub iso639p1: String,
    
    pub name: String,

    pub family: LanguageFamily,

    /// Syntactic properties (word order, headedness, flexibility).
    pub syntax: Option<SyntaxFeatures>,

    /// Morphological characteristics (inflection, typology).
    pub morphology: Option<MorphologyFeatures>,

    /// Nominal system properties (gender, case, classifiers).
    pub nominal: Option<NominalFeatures>,

    /// Verbal system properties (tense, aspect, agreement).
    pub verbal: Option<VerbalFeatures>,

    /// Discourse and pragmatic features (topic prominence, pro-drop).
    pub discourse: Option<DiscourseFeatures>,

    /// Phonological characteristics (tone, pitch accent).
    pub phonology: Option<PhonologyFeatures>,

    /// Writing system and script information.
    pub writing: Option<WritingFeatures>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LanguageFamily {
    pub family: String,
    pub subfamily: Option<String>,
    pub branch: Option<String>,
    pub group: Option<String>,
    pub subgroup: Option<String>,
}

/// Syntactic word order and clause-structure properties.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxFeatures {
    /// All attested basic clause word orders.
    ///
    /// Examples:
    /// - ["SVO"]
    /// - ["SOV"]
    /// - ["V2", "SOV"]
    pub word_orders: Vec<String>,

    /// The dominant or unmarked word order.
    pub dominant_order: Option<String>,

    /// Head-directionality of phrases.
    ///
    /// Common values:
    /// - "head-initial"
    /// - "head-final"
    /// - "mixed"
    pub head_direction: Option<String>,

    /// Degree of syntactic word-order rigidity.
    ///
    /// Common values:
    /// - "fixed"
    /// - "conditional"
    /// - "free"
    pub flexibility: Option<String>,

    /// Whether the language exhibits verb-second (V2) behavior.
    ///
    /// Typical for Germanic languages.
    pub verb_second: Option<bool>,

    /// Position of the verb in subordinate clauses, if distinct.
    ///
    /// Examples: "final", "second", "initial"
    pub subordinate_clause_verb_position: Option<String>,

    /// Whether the language is topic-prominent rather than subject-prominent.
    pub topic_prominent: Option<bool>,

    /// Whether grammatical relations are primarily encoded by word order
    /// rather than morphology (cases, agreement).
    pub relies_on_word_order: Option<bool>,

    /// Whether the language allows argument scrambling.
    ///
    /// Common in Japanese, Korean, German.
    pub scrambling: Option<bool>,

    /// Whether null constituents (subjects or objects) are syntactically licensed.
    pub null_arguments: Option<bool>,

    /// Free-form explanatory notes or edge cases.
    pub notes: Option<String>,
}

/// Morphological structure and word-formation properties.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MorphologyFeatures {
    /// Morphological typology of the language.
    ///
    /// Examples: "analytic", "agglutinative", "fusional", "polysynthetic"
    pub typology: Option<String>,

    /// Whether the language uses inflectional morphology.
    pub inflection: Option<bool>,

    /// Whether verbs conjugate for grammatical categories.
    pub conjugation: Option<bool>,

    /// Whether productive derivational morphology exists.
    pub derivation: Option<bool>,

    /// Free-form notes on morphological behavior.
    pub notes: Option<String>,
}

/// Properties of nouns and noun phrases.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NominalFeatures {
    /// Whether nouns participate in a grammatical gender or class system.
    pub grammatical_gender: Option<bool>,

    /// Types of gender or noun classes used.
    ///
    /// Examples: ["masculine", "feminine"], ["animate", "inanimate"]
    pub gender_types: Option<Vec<String>>,

    /// Grammatical cases expressed in the language.
    ///
    /// Examples: ["nominative", "accusative"], ["ergative", "absolutive"]
    pub cases: Option<Vec<String>>,

    /// Whether the language uses classifiers.
    pub classifiers: Option<bool>,

    /// Type of classifier system.
    ///
    /// Examples: "numeral", "noun-class"
    pub classifier_type: Option<String>,
}

/// Verbal categories and grammatical marking.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VerbalFeatures {
    /// Presence of grammatical tense marking.
    pub tense: Option<bool>,

    /// Presence of grammatical aspect marking.
    pub aspect: Option<bool>,

    /// Presence of grammatical mood or modality.
    pub mood: Option<bool>,

    /// Agreement categories marked on the verb.
    ///
    /// Examples: ["person", "number"], ["gender"]
    pub agreement: Option<Vec<String>>,

    /// Whether auxiliary verbs are used.
    pub auxiliaries: Option<bool>,

    /// Additional explanatory notes.
    pub notes: Option<String>,
}

/// Discourse-level and pragmatic features.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiscourseFeatures {
    /// Whether the language is topic-prominent rather than subject-prominent.
    pub topic_prominent: Option<bool>,

    /// Whether subject pronouns can be omitted pragmatically.
    pub pro_drop: Option<bool>,

    /// Whether a formal politeness or honorific system exists.
    pub politeness_system: Option<bool>,

    /// Named honorific or politeness levels, if applicable.
    pub honorific_levels: Option<Vec<String>>,
}

/// Phonological properties of the language.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PhonologyFeatures {
    /// Whether lexical tone distinguishes meaning.
    pub tonal: Option<bool>,

    /// Number of contrastive tones, if tonal.
    pub tone_count: Option<u8>,

    /// Whether the language uses pitch accent.
    pub pitch_accent: Option<bool>,

    /// Whether stress is phonemic.
    pub stress: Option<bool>,
}

/// Writing systems and scripts used by the language.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WritingFeatures {
    /// Scripts used by the language.
    ///
    /// Examples: ["Latin"], ["Han", "Kana"]
    pub scripts: Vec<String>,

    /// Script directions used.
    ///
    /// Examples: ["LTR"], ["RTL"], ["vertical"]
    pub direction: Vec<String>,

    /// Whether the script is alphabetic.
    pub alphabetic: Option<bool>,

    /// Whether the script is an abugida.
    pub abugida: Option<bool>,

    /// Whether the script is logographic.
    pub logographic: Option<bool>,
}