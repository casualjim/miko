use async_openai::types::{
  CreateEmbeddingRequest, DeleteFileResponse, ListFilesResponse, OpenAIFile,
};
use axum::{
  extract::{Query, State},
  response::IntoResponse,
  routing::post,
  Json,
};

use crate::{app::state::AppState, models, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/", get(list_files).post(create))
    .route("/:file_id", get(get_file).delete(delete_file))
    .route("/:file_id/content", get(get_file_content))
    .with_state(app_state)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn create(
  State(app_state): State<AppState>,
  mut multipart: axum::extract::Multipart,
) -> Result<Json<OpenAIFile>> {
  app_state
    .openai_client()
    .files()
    .create(params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[derive(Debug, Serialize, Deserialize)]
struct ListFilesRequest {
  pub purpose: Option<String>,
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn list_files(
  State(app_state): State<AppState>,
  Query(params): Query<ListFilesRequest>,
) -> Result<Json<ListFilesResponse>> {
  app_state
    .openai_client()
    .files()
    .list(&params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn get_file(
  State(app_state): State<AppState>,
  Path(file_id): Path<String>,
) -> Result<Json<OpenAIFile>> {
  app_state
    .openai_client()
    .files()
    .retrieve(&file_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn delete_file(
  State(app_state): State<AppState>,
  Path(file_id): Path<String>,
) -> Result<Json<DeleteFileResponse>> {
  app_state
    .openai_client()
    .files()
    .delete(&file_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn get_file_content(
  State(app_state): State<AppState>,
  Path(file_id): Path<String>,
) -> Result<String> {
  app_state
    .openai_client()
    .files()
    .retrieve_content(&file_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
