use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::ChatError;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder, PartialEq)]
#[builder(name = "CreateImageRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct CreateImageRequest {
  /// A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2`
  /// and 4000 characters for `dall-e-3`.
  pub prompt: String,

  /// The model to use for image generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub model: Option<ImageModel>,

  /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub n: Option<u8>, // min:1 max:10 default:1

  /// The quality of the image that will be generated. `hd` creates images with finer details and greater
  /// consistency across the image. This param is only supported for `dall-e-3`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quality: Option<ImageQuality>,

  /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub response_format: Option<ResponseFormat>,

  /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`.
  /// Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub size: Option<ImageSize>,

  /// The style of the generated images. Must be one of `vivid` or `natural`.
  /// Vivid causes the model to lean towards generating hyper-real and dramatic images.
  /// Natural causes the model to produce more natural, less hyper-real looking images.
  /// This param is only supported for `dall-e-3`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<ImageStyle>,

  /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::CreateImageRequest> for CreateImageRequest {
  fn from(req: async_openai::types::CreateImageRequest) -> Self {
    Self {
      prompt: req.prompt,
      model: req.model.map(|m| m.into()),
      n: req.n,
      quality: req.quality.map(|q| q.into()),
      response_format: req.response_format.map(|rf| rf.into()),
      size: req.size.map(|s| s.into()),
      style: req.style.map(|s| s.into()),
      user: req.user,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<CreateImageRequest> for async_openai::types::CreateImageRequest {
  fn from(req: CreateImageRequest) -> Self {
    Self {
      prompt: req.prompt,
      model: req.model.map(|m| m.into()),
      n: req.n,
      quality: req.quality.map(|q| q.into()),
      response_format: req.response_format.map(|rf| rf.into()),
      size: req.size.map(|s| s.into()),
      style: req.style.map(|s| s.into()),
      user: req.user,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum ImageModel {
  #[default]
  #[serde(rename = "dall-e-2")]
  DallE2,
  #[serde(rename = "dall-e-3")]
  DallE3,
  #[serde(untagged)]
  Other(String),
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::ImageModel> for ImageModel {
  fn from(req: async_openai::types::ImageModel) -> Self {
    match req {
      async_openai::types::ImageModel::DallE2 => ImageModel::DallE2,
      async_openai::types::ImageModel::DallE3 => ImageModel::DallE3,
      async_openai::types::ImageModel::Other(s) => ImageModel::Other(s),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<ImageModel> for async_openai::types::ImageModel {
  fn from(req: ImageModel) -> Self {
    match req {
      ImageModel::DallE2 => async_openai::types::ImageModel::DallE2,
      ImageModel::DallE3 => async_openai::types::ImageModel::DallE3,
      ImageModel::Other(s) => async_openai::types::ImageModel::Other(s),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
  #[default]
  Standard,
  HD,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::ImageQuality> for ImageQuality {
  fn from(req: async_openai::types::ImageQuality) -> Self {
    match req {
      async_openai::types::ImageQuality::Standard => ImageQuality::Standard,
      async_openai::types::ImageQuality::HD => ImageQuality::HD,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<ImageQuality> for async_openai::types::ImageQuality {
  fn from(req: ImageQuality) -> Self {
    match req {
      ImageQuality::Standard => async_openai::types::ImageQuality::Standard,
      ImageQuality::HD => async_openai::types::ImageQuality::HD,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
  #[default]
  Vivid,
  Natural,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::ImageStyle> for ImageStyle {
  fn from(req: async_openai::types::ImageStyle) -> Self {
    match req {
      async_openai::types::ImageStyle::Vivid => ImageStyle::Vivid,
      async_openai::types::ImageStyle::Natural => ImageStyle::Natural,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<ImageStyle> for async_openai::types::ImageStyle {
  fn from(req: ImageStyle) -> Self {
    match req {
      ImageStyle::Vivid => async_openai::types::ImageStyle::Vivid,
      ImageStyle::Natural => async_openai::types::ImageStyle::Natural,
    }
  }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ImageSize {
  #[serde(rename = "256x256")]
  S256x256,
  #[serde(rename = "512x512")]
  S512x512,
  #[default]
  #[serde(rename = "1024x1024")]
  S1024x1024,
  #[serde(rename = "1792x1024")]
  S1792x1024,
  #[serde(rename = "1024x1792")]
  S1024x1792,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::ImageSize> for ImageSize {
  fn from(req: async_openai::types::ImageSize) -> Self {
    match req {
      async_openai::types::ImageSize::S256x256 => ImageSize::S256x256,
      async_openai::types::ImageSize::S512x512 => ImageSize::S512x512,
      async_openai::types::ImageSize::S1024x1024 => ImageSize::S1024x1024,
      async_openai::types::ImageSize::S1792x1024 => ImageSize::S1792x1024,
      async_openai::types::ImageSize::S1024x1792 => ImageSize::S1024x1792,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<ImageSize> for async_openai::types::ImageSize {
  fn from(req: ImageSize) -> Self {
    match req {
      ImageSize::S256x256 => async_openai::types::ImageSize::S256x256,
      ImageSize::S512x512 => async_openai::types::ImageSize::S512x512,
      ImageSize::S1024x1024 => async_openai::types::ImageSize::S1024x1024,
      ImageSize::S1792x1024 => async_openai::types::ImageSize::S1792x1024,
      ImageSize::S1024x1792 => async_openai::types::ImageSize::S1024x1792,
    }
  }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DallE2ImageSize {
  #[serde(rename = "256x256")]
  S256x256,
  #[serde(rename = "512x512")]
  S512x512,
  #[default]
  #[serde(rename = "1024x1024")]
  S1024x1024,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::DallE2ImageSize> for DallE2ImageSize {
  fn from(req: async_openai::types::DallE2ImageSize) -> Self {
    match req {
      async_openai::types::DallE2ImageSize::S256x256 => DallE2ImageSize::S256x256,
      async_openai::types::DallE2ImageSize::S512x512 => DallE2ImageSize::S512x512,
      async_openai::types::DallE2ImageSize::S1024x1024 => DallE2ImageSize::S1024x1024,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<DallE2ImageSize> for async_openai::types::DallE2ImageSize {
  fn from(req: DallE2ImageSize) -> Self {
    match req {
      DallE2ImageSize::S256x256 => async_openai::types::DallE2ImageSize::S256x256,
      DallE2ImageSize::S512x512 => async_openai::types::DallE2ImageSize::S512x512,
      DallE2ImageSize::S1024x1024 => async_openai::types::DallE2ImageSize::S1024x1024,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormat {
  #[default]
  Url,
  #[serde(rename = "b64_json")]
  B64Json,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::ResponseFormat> for ResponseFormat {
  fn from(req: async_openai::types::ResponseFormat) -> Self {
    match req {
      async_openai::types::ResponseFormat::Url => ResponseFormat::Url,
      async_openai::types::ResponseFormat::B64Json => ResponseFormat::B64Json,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<ResponseFormat> for async_openai::types::ResponseFormat {
  fn from(req: ResponseFormat) -> Self {
    match req {
      ResponseFormat::Url => async_openai::types::ResponseFormat::Url,
      ResponseFormat::B64Json => async_openai::types::ResponseFormat::B64Json,
    }
  }
}
