use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use utoipa::{openapi, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::language::get_language_metadata;
use crate::routes::pronouns::get_personal_pronouns;
use crate::routes::dictionary::get_lexical_entry;
use crate::routes::verbs::get_verbs;
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        
    ),
    tags(
        (name = "lang", description = "Lang API")
    )
)]
struct ApiDoc;

pub fn setup_routing(state: AppState) -> Router {

    let sub_router: OpenApiRouter = OpenApiRouter::new()
        .fallback(handler_404)
        .route("/{lang}", get(get_language_metadata))
        .route("/{lang}/verbs", get(get_verbs))
        .route("/{lang}/personal-pronouns", get(get_personal_pronouns))
        .route("/dictionary/{from}/{to}/{word}", get(get_lexical_entry))
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
