use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, Router};
use utoipa::{openapi, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::Pronouns;
use crate::{routes::*, state::AppState};

#[derive(OpenApi)]
#[openapi(
    paths(
        
    ),
    tags(
        (name = "lang", description = "Lang API")
    )
)]
struct ApiDoc;

pub async fn get_personal_pronouns(State(state): State<AppState>, lang: Path<String>) -> Json<Option<Pronouns>> {
    
    let document = state.pronouns.get(&lang).await.unwrap();
    
    Json(document)
}

pub fn setup_routing(state: AppState) -> Router {

    let sub_router: OpenApiRouter = OpenApiRouter::new()
        .fallback(handler_404)
        .route("/", get(get_personal_pronouns))
        .with_state(state);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", sub_router)
        .split_for_parts();

    router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}
