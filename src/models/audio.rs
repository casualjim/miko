use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::ChatError;

#[derive(Clone, Default, Debug, Builder, PartialEq, Serialize, Deserialize)]
#[builder(name = "CreateSpeechRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct CreateSpeechRequest {
  /// The text to generate audio for. The maximum length is 4096 characters.
  pub input: String,

  /// One of the available [TTS models](https://platform.openai.com/docs/models/tts): `tts-1` or `tts-1-hd`
  pub model: SpeechModel,

  /// The voice to use when generating the audio. Supported voices are `alloy`, `echo`, `fable`, `onyx`, `nova`, and `shimmer`. Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech/voice-options).
  pub voice: Voice,

  /// The format to audio in. Supported formats are mp3, opus, aac, and flac.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub response_format: Option<SpeechResponseFormat>,

  /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub speed: Option<f32>, // default: 1.0
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::CreateSpeechRequest> for CreateSpeechRequest {
  fn from(v: async_openai::types::CreateSpeechRequest) -> Self {
    Self {
      input: v.input,
      model: v.model.into(),
      voice: v.voice.into(),
      response_format: v.response_format.map(Into::into),
      speed: v.speed,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<CreateSpeechRequest> for async_openai::types::CreateSpeechRequest {
  fn from(v: CreateSpeechRequest) -> Self {
    Self {
      input: v.input,
      model: v.model.into(),
      voice: v.voice.into(),
      response_format: v.response_format.map(Into::into),
      speed: v.speed,
    }
  }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SpeechModel {
  #[default]
  #[serde(rename = "tts-1")]
  Tts1,
  #[serde(rename = "tts-1-hd")]
  Tts1Hd,
  #[serde(untagged)]
  Other(String),
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::SpeechModel> for SpeechModel {
  fn from(v: async_openai::types::SpeechModel) -> Self {
    match v {
      async_openai::types::SpeechModel::Tts1 => Self::Tts1,
      async_openai::types::SpeechModel::Tts1Hd => Self::Tts1Hd,
      async_openai::types::SpeechModel::Other(s) => Self::Other(s),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<SpeechModel> for async_openai::types::SpeechModel {
  fn from(v: SpeechModel) -> Self {
    match v {
      SpeechModel::Tts1 => Self::Tts1,
      SpeechModel::Tts1Hd => Self::Tts1Hd,
      SpeechModel::Other(s) => Self::Other(s),
    }
  }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Voice {
  #[default]
  Alloy,
  Echo,
  Fable,
  Onyx,
  Nova,
  Shimmer,
  #[serde(untagged)]
  Other(String),
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::Voice> for Voice {
  fn from(v: async_openai::types::Voice) -> Self {
    match v {
      async_openai::types::Voice::Echo => Self::Echo,
      async_openai::types::Voice::Fable => Self::Fable,
      async_openai::types::Voice::Onyx => Self::Onyx,
      async_openai::types::Voice::Nova => Self::Nova,
      async_openai::types::Voice::Shimmer => Self::Shimmer,
      async_openai::types::Voice::Other(s) => Self::Other(s),
      _ => Self::Alloy,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<Voice> for async_openai::types::Voice {
  fn from(v: Voice) -> Self {
    match v {
      Voice::Echo => Self::Echo,
      Voice::Fable => Self::Fable,
      Voice::Onyx => Self::Onyx,
      Voice::Nova => Self::Nova,
      Voice::Shimmer => Self::Shimmer,
      Voice::Other(s) => Self::Other(s),
      _ => Self::Alloy,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SpeechResponseFormat {
  #[default]
  Mp3,
  Opus,
  Aac,
  Flac,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::SpeechResponseFormat> for SpeechResponseFormat {
  fn from(v: async_openai::types::SpeechResponseFormat) -> Self {
    match v {
      async_openai::types::SpeechResponseFormat::Mp3 => Self::Mp3,
      async_openai::types::SpeechResponseFormat::Opus => Self::Opus,
      async_openai::types::SpeechResponseFormat::Aac => Self::Aac,
      async_openai::types::SpeechResponseFormat::Flac => Self::Flac,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<SpeechResponseFormat> for async_openai::types::SpeechResponseFormat {
  fn from(v: SpeechResponseFormat) -> Self {
    match v {
      SpeechResponseFormat::Mp3 => Self::Mp3,
      SpeechResponseFormat::Opus => Self::Opus,
      SpeechResponseFormat::Aac => Self::Aac,
      SpeechResponseFormat::Flac => Self::Flac,
    }
  }
}
