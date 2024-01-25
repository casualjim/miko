use async_openai::types::{
  CreateEditRequest, CreateEmbeddingRequest, CreateImageEditRequest, CreateImageRequest,
  CreateImageVariationRequest, Image,
};
use axum::{extract::State, response::IntoResponse, routing::post, Json};

use crate::{app::state::AppState, models, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/generations", post(create))
    .route("/edits", post(edit))
    .route("/variations", post(variations))
    .with_state(app_state)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn create(
  State(app_state): State<AppState>,
  Json(params): Json<CreateImageRequest>,
) -> Result<Json<Image>> {
  app_state
    .openai_client()
    .images()
    .create(params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn edit(
  State(app_state): State<AppState>,
  Json(params): Json<CreateImageEditRequest>,
) -> Result<Json<Image>> {
  app_state
    .openai_client()
    .images()
    .create_edit(params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn create(
  State(app_state): State<AppState>,
  Json(params): Json<CreateImageVariationRequest>,
) -> Result<Json<Image>> {
  app_state
    .openai_client()
    .images()
    .create_variation(params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
