use async_openai::types::{CreateModerationRequest, CreateModerationResponse};
use axum::{extract::State, response::IntoResponse, routing::post, Json};

use crate::{app::state::AppState, models, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/", post(create))
    .with_state(app_state)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn create(
  State(app_state): State<AppState>,
  Json(params): Json<CreateModerationRequest>,
) -> Result<Json<CreateModerationResponse>> {
  app_state
    .openai_client()
    .moderations()
    .create(params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}