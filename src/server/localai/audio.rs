use async_openai::types::{
  AudioInput, CreateTranscriptionRequest, CreateTranscriptionResponse, CreateTranslationRequest,
  CreateTranslationResponse,
};
use axum::{
  extract::{Json, Multipart, State},
  routing::post,
};
use bytes::Bytes;

use crate::{app::state::AppState, models::audio::CreateSpeechRequest, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/speech", post(speech))
    .route("/transcriptions", post(transcriptions))
    .route("/translations", post(translations))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state))]
async fn speech(
  State(app_state): State<AppState>,
  Json(request): Json<CreateSpeechRequest>,
) -> Result<Bytes> {
  app_state
    .openai_client()
    .audio()
    .speech(request.into())
    .await
    .map_err(Into::into)
    .map(|v| v.bytes)
}

async fn create_transcription_request(
  mut request: Multipart,
) -> Result<CreateTranscriptionRequest> {
  let mut req = CreateTranscriptionRequest::default();
  while let Ok(Some(field)) = request.next_field().await {
    if let Some(field_name) = field.name() {
      match field_name {
        "file" => {
          req.file = AudioInput {
            source: async_openai::types::InputSource::Bytes {
              filename: field.file_name().unwrap_or_default().to_string(),
              bytes: field
                .bytes()
                .await
                .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?,
            },
          }
        }
        "model" => {
          req.model = field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
        }
        "prompt" => req.prompt = field.text().await.ok(),
        "response_format" => {
          req.response_format = field.text().await.ok().map(|v| match v.as_str() {
            "json" => async_openai::types::AudioResponseFormat::Json,
            "text" => async_openai::types::AudioResponseFormat::Text,
            "srt" => async_openai::types::AudioResponseFormat::Srt,
            "verbose_json" => async_openai::types::AudioResponseFormat::VerboseJson,
            "vtt" => async_openai::types::AudioResponseFormat::Vtt,
            _ => async_openai::types::AudioResponseFormat::Json,
          })
        }
        "temperature" => req.temperature = field.text().await.ok().and_then(|v| v.parse().ok()),
        "language" => req.language = field.text().await.ok(),
        _ => {}
      }
    }
  }
  Ok(req)
}

#[tracing::instrument(skip(app_state))]
async fn transcriptions(
  State(app_state): State<AppState>,
  request: Multipart,
) -> Result<Json<CreateTranscriptionResponse>> {
  app_state
    .openai_client()
    .audio()
    .transcribe(create_transcription_request(request).await?)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

async fn create_translation_request(mut request: Multipart) -> Result<CreateTranslationRequest> {
  let mut req = CreateTranslationRequest::default();
  while let Ok(Some(field)) = request.next_field().await {
    if let Some(field_name) = field.name() {
      match field_name {
        "file" => {
          req.file = AudioInput {
            source: async_openai::types::InputSource::Bytes {
              filename: field.file_name().unwrap_or_default().to_string(),
              bytes: field
                .bytes()
                .await
                .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?,
            },
          }
        }
        "model" => {
          req.model = field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
        }
        "prompt" => req.prompt = field.text().await.ok(),
        "response_format" => {
          req.response_format = field.text().await.ok().map(|v| match v.as_str() {
            "json" => async_openai::types::AudioResponseFormat::Json,
            "text" => async_openai::types::AudioResponseFormat::Text,
            "srt" => async_openai::types::AudioResponseFormat::Srt,
            "verbose_json" => async_openai::types::AudioResponseFormat::VerboseJson,
            "vtt" => async_openai::types::AudioResponseFormat::Vtt,
            _ => async_openai::types::AudioResponseFormat::Json,
          })
        }
        "temperature" => req.temperature = field.text().await.ok().and_then(|v| v.parse().ok()),
        _ => {}
      }
    }
  }
  Ok(req)
}

#[tracing::instrument(skip(app_state))]
async fn translations(
  State(app_state): State<AppState>,
  request: Multipart,
) -> Result<Json<CreateTranslationResponse>> {
  app_state
    .openai_client()
    .audio()
    .translate(create_translation_request(request).await?)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
