use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::ChatError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ModerationInput {
  String(String),
  StringArray(Vec<String>),
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::ModerationInput> for ModerationInput {
  fn from(input: async_openai::types::ModerationInput) -> Self {
    match input {
      async_openai::types::ModerationInput::String(s) => Self::String(s),
      async_openai::types::ModerationInput::StringArray(s) => Self::StringArray(s),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<ModerationInput> for async_openai::types::ModerationInput {
  fn from(input: ModerationInput) -> Self {
    match input {
      ModerationInput::String(s) => Self::String(s),
      ModerationInput::StringArray(s) => Self::StringArray(s),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
pub enum TextModerationModel {
  #[default]
  #[serde(rename = "text-moderation-latest")]
  Latest,
  #[serde(rename = "text-moderation-stable")]
  Stable,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::TextModerationModel> for TextModerationModel {
  fn from(model: async_openai::types::TextModerationModel) -> Self {
    match model {
      async_openai::types::TextModerationModel::Latest => Self::Latest,
      async_openai::types::TextModerationModel::Stable => Self::Stable,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<TextModerationModel> for async_openai::types::TextModerationModel {
  fn from(model: TextModerationModel) -> Self {
    match model {
      TextModerationModel::Latest => Self::Latest,
      TextModerationModel::Stable => Self::Stable,
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder, PartialEq)]
#[builder(name = "CreateModerationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct CreateModerationRequest {
  /// The input text to classify
  pub input: Option<ModerationInput>,

  /// Two content moderations models are available: `text-moderation-stable` and `text-moderation-latest`.
  ///
  /// The default is `text-moderation-latest` which will be automatically upgraded over time. This ensures you are always using our most accurate model. If you use `text-moderation-stable`, we will provide advanced notice before updating the model. Accuracy of `text-moderation-stable` may be slightly lower than for `text-moderation-latest`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub model: Option<TextModerationModel>,
}

#[cfg(feature = "ssr")]
impl From<CreateModerationRequest> for async_openai::types::CreateModerationRequest {
  fn from(request: CreateModerationRequest) -> Self {
    Self {
      input: request
        .input
        .map(Into::into)
        .unwrap_or_else(|| async_openai::types::ModerationInput::String("".to_string())),
      model: request.model.map(Into::into),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::CreateModerationRequest> for CreateModerationRequest {
  fn from(request: async_openai::types::CreateModerationRequest) -> Self {
    Self {
      input: Some(request.input.into()),
      model: request.model.map(Into::into),
    }
  }
}
