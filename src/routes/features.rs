use axum::extract::{Path, State};
use axum::Json;

use crate::models::{LanguageFeatures, Pronouns};
use crate::{routes::*, state::AppState};

pub async fn get_features(State(state): State<AppState>, lang: Path<String>) -> Json<Option<LanguageFeatures>> {
    
    let document = state.features.get(&lang).await.unwrap();
    
    Json(document)
}