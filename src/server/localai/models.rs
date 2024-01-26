use async_openai::types::{DeleteModelResponse, ListModelResponse, Model};
use axum::{
  extract::{Path, State},
  routing::get,
  Json,
};

use crate::{app::state::AppState, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/", get(list_models))
    .route("/:model_id", get(get_model).delete(delete_model))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state))]
async fn list_models(State(app_state): State<AppState>) -> Result<Json<ListModelResponse>> {
  let models = app_state.openai_client().models().list().await?;
  Ok(Json(models))
}

#[tracing::instrument(skip(app_state))]
async fn get_model(
  State(app_state): State<AppState>,
  Path(model_id): Path<String>,
) -> Result<Json<Model>> {
  app_state
    .openai_client()
    .models()
    .retrieve(&model_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[tracing::instrument(skip(app_state))]
async fn delete_model(
  State(app_state): State<AppState>,
  Path(model_id): Path<String>,
) -> Result<Json<DeleteModelResponse>> {
  app_state
    .openai_client()
    .models()
    .delete(&model_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
