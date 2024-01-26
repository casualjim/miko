use async_openai::types::{
  CreateFileRequest, DeleteFileResponse, FileInput, ListFilesResponse, OpenAIFile,
};
use axum::{
  extract::{Multipart, Path, Query, State},
  routing::get,
  Json,
};
use serde::{Deserialize, Serialize};

use crate::{app::state::AppState, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/", get(list_files).post(create))
    .route("/:file_id", get(get_file).delete(delete_file))
    .route("/:file_id/content", get(get_file_content))
    .with_state(app_state)
}

async fn create_file_request(mut request: Multipart) -> Result<CreateFileRequest> {
  let mut req = CreateFileRequest::default();
  while let Ok(Some(field)) = request.next_field().await {
    if let Some(field_name) = field.name() {
      match field_name {
        "file" => {
          req.file = FileInput {
            source: async_openai::types::InputSource::Bytes {
              filename: field.file_name().unwrap_or_default().to_string(),
              bytes: field
                .bytes()
                .await
                .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?,
            },
          }
        }
        "purpose" => {
          req.purpose = field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
        }
        _ => {}
      }
    }
  }
  Ok(req)
}

#[tracing::instrument(skip(app_state))]
async fn create(
  State(app_state): State<AppState>,
  multipart: axum::extract::Multipart,
) -> Result<Json<OpenAIFile>> {
  app_state
    .openai_client()
    .files()
    .create(create_file_request(multipart).await?)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[derive(Debug, Serialize, Deserialize)]
struct ListFilesRequest {
  pub purpose: Option<String>,
}

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
