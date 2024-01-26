use axum::{extract::State, routing::post, Json};

use crate::{
  app::state::AppState,
  models::embeddings::{CreateEmbeddingRequest, CreateEmbeddingResponse},
  Result,
};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/", post(create))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state))]
async fn create(
  State(app_state): State<AppState>,
  Json(params): Json<CreateEmbeddingRequest>,
) -> Result<Json<CreateEmbeddingResponse>> {
  app_state
    .openai_client()
    .embeddings()
    .create(params.into())
    .await
    .map_err(Into::into)
    .map(|v| Json(v.into()))
}
