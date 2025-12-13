use axum::extract::{Path, State};
use axum::Json;

use crate::models::*;
use crate::state::AppState;

pub async fn get_personal_pronouns(State(state): State<AppState>, lang: Path<String>) -> Json<Option<PersonalPronouns>> {
    
    let document = state.pronouns.get(&lang).await.unwrap();
    
    Json(document)
}