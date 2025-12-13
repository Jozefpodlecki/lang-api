use axum::extract::{Path, State};
use axum::Json;

use crate::models::*;
use crate::state::AppState;

pub async fn get_verbs(State(state): State<AppState>, lang: Path<String>) -> Json<Vec<Verb>> {
    
    let documents = state.verbs.get_all(&lang).await.unwrap();
    
    Json(documents)
}