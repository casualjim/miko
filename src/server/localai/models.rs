use async_openai::types::{
  CreateEditRequest, CreateEmbeddingRequest, CreateImageEditRequest, CreateImageRequest,
  CreateImageVariationRequest, Image,
};
use axum::{
  extract::{Path, State},
  response::IntoResponse,
  routing::post,
  Json,
};

use crate::{app::state::AppState, models, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/", get(list_models))
    .route("/:model_id", get(get_model).delete(delete_model))
    .with_state(app_state)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn list_models(State(app_state): State<AppState>) -> Result<impl IntoResponse> {
  let models = app_state.openai_client().models().list().await?;
  Ok(Json(models))
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn get_model(
  State(app_state): State<AppState>,
  Path(model_id): Path<String>,
) -> Result<impl IntoResponse> {
  app_state
    .openai_client()
    .models()
    .retrieve(&model_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn get_model(
  State(app_state): State<AppState>,
  Path(model_id): Path<String>,
) -> Result<impl IntoResponse> {
  app_state
    .openai_client()
    .models()
    .delete(&model_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
