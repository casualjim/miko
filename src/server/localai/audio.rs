use async_openai::types::{
  CreateSpeechRequest, CreateTranscriptionRequest, CreateTranslationRequest,
};
use axum::{
  extract::{Json, State},
  response::IntoResponse,
};

use crate::{state::AppState, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/speech", post(speech))
    .route("/transcriptions", post(transcriptions))
    .route("/translations", post(translations))
    .with_state(app_state)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn speech(
  State(app_state): State<AppState>,
  Json(request): Json<CreateSpeechRequest>,
) -> Result<impl IntoResponse> {
  app_state
    .openai_client()
    .audio()
    .speech(request)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn transcriptions(
  State(app_state): State<AppState>,
  Json(request): Json<CreateTranscriptionRequest>,
) -> Result<impl IntoResponse> {
  app_state
    .openai_client()
    .audio()
    .transcribe(request)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn translations(
  State(app_state): State<AppState>,
  Json(request): Json<CreateTranslationRequest>,
) -> Result<impl IntoResponse> {
  app_state
    .openai_client()
    .audio()
    .translate(request)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
