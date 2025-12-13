use axum::extract::{Path, State};
use axum::Json;

use crate::models::*;
use crate::state::AppState;

pub async fn get_lexical_entry(State(state): State<AppState>, lang: Path<String>) -> Json<Option<LanguageFeatures>> {
    
    let document = state.features.get(&lang).await.unwrap();
    
    Json(document)
}