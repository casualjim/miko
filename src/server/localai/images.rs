use async_openai::types::{
  CreateImageEditRequest, CreateImageVariationRequest, ImageInput, ImagesResponse,
};
use axum::{
  extract::{Multipart, State},
  routing::post,
  Json,
};

use crate::{app::state::AppState, models::images::CreateImageRequest, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/generations", post(create))
    .route("/edits", post(edit))
    .route("/variations", post(variations))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state))]
async fn create(
  State(app_state): State<AppState>,
  Json(params): Json<CreateImageRequest>,
) -> Result<Json<ImagesResponse>> {
  app_state
    .openai_client()
    .images()
    .create(params.into())
    .await
    .map_err(Into::into)
    .map(Into::into)
}

async fn create_image_edit_request(mut request: Multipart) -> Result<CreateImageEditRequest> {
  let mut req = CreateImageEditRequest::default();
  while let Ok(Some(field)) = request.next_field().await {
    if let Some(field_name) = field.name() {
      match field_name {
        "image" => {
          req.image = async_openai::types::ImageInput {
            source: async_openai::types::InputSource::Bytes {
              filename: field.file_name().unwrap_or_default().to_string(),
              bytes: field
                .bytes()
                .await
                .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?,
            },
          }
        }
        "prompt" => {
          req.prompt = field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
        }
        "mask" => {
          req.mask = Some(ImageInput {
            source: async_openai::types::InputSource::Bytes {
              filename: field.file_name().unwrap_or_default().to_string(),
              bytes: field
                .bytes()
                .await
                .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?,
            },
          })
        }
        "model" => {
          req.model = Some(
            match field
              .text()
              .await
              .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
              .as_str()
            {
              "dall-e-3" => async_openai::types::ImageModel::DallE3,
              "dall-e-2" => async_openai::types::ImageModel::DallE2,
              s if !s.is_empty() => async_openai::types::ImageModel::Other(s.to_string()),
              _ => async_openai::types::ImageModel::DallE2,
            },
          )
        }
        "n" => {
          req.n = field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
            .parse()
            .ok()
        }
        "size" => {
          req.size = match field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
            .as_str()
          {
            "256x256" => Some(async_openai::types::DallE2ImageSize::S256x256),
            "512x512" => Some(async_openai::types::DallE2ImageSize::S512x512),
            "1024x1024" => Some(async_openai::types::DallE2ImageSize::S1024x1024),
            _ => None,
          }
        }
        "response_format" => {
          req.response_format = match field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
            .as_str()
          {
            "url" => Some(async_openai::types::ResponseFormat::Url),
            "b64_json" => Some(async_openai::types::ResponseFormat::B64Json),
            _ => None,
          }
        }
        "user" => {
          req.user = Some(
            field
              .text()
              .await
              .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?,
          )
        }
        _ => {}
      }
    }
  }
  Ok(req)
}

#[tracing::instrument(skip(app_state))]
async fn edit(
  State(app_state): State<AppState>,
  request: Multipart,
) -> Result<Json<ImagesResponse>> {
  app_state
    .openai_client()
    .images()
    .create_edit(create_image_edit_request(request).await?)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

async fn create_variation_request(mut request: Multipart) -> Result<CreateImageVariationRequest> {
  let mut req = CreateImageVariationRequest::default();
  while let Ok(Some(field)) = request.next_field().await {
    if let Some(field_name) = field.name() {
      match field_name {
        "image" => {
          req.image = async_openai::types::ImageInput {
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
          req.model = Some(
            match field
              .text()
              .await
              .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
              .as_str()
            {
              "dall-e-3" => async_openai::types::ImageModel::DallE3,
              "dall-e-2" => async_openai::types::ImageModel::DallE2,
              s if !s.is_empty() => async_openai::types::ImageModel::Other(s.to_string()),
              _ => async_openai::types::ImageModel::DallE2,
            },
          )
        }
        "n" => {
          req.n = field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
            .parse()
            .ok()
        }
        "size" => {
          req.size = match field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
            .as_str()
          {
            "256x256" => Some(async_openai::types::DallE2ImageSize::S256x256),
            "512x512" => Some(async_openai::types::DallE2ImageSize::S512x512),
            "1024x1024" => Some(async_openai::types::DallE2ImageSize::S1024x1024),
            _ => None,
          }
        }
        "response_format" => {
          req.response_format = match field
            .text()
            .await
            .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?
            .as_str()
          {
            "url" => Some(async_openai::types::ResponseFormat::Url),
            "b64_json" => Some(async_openai::types::ResponseFormat::B64Json),
            _ => None,
          }
        }
        "user" => {
          req.user = Some(
            field
              .text()
              .await
              .map_err(|e| crate::Error::InvalidArgument(e.to_string()))?,
          )
        }
        _ => {}
      }
    }
  }
  Ok(req)
}

#[tracing::instrument(skip(app_state))]
async fn variations(
  State(app_state): State<AppState>,
  request: Multipart,
) -> Result<Json<ImagesResponse>> {
  app_state
    .openai_client()
    .images()
    .create_variation(create_variation_request(request).await?)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
