use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::ChatError;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum EmbeddingInput {
  String(String),
  StringArray(Vec<String>),
  // Minimum value is 0, maximum value is 100257 (inclusive).
  IntegerArray(Vec<u32>),
  ArrayOfIntegerArray(Vec<Vec<u32>>),
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncodingFormat {
  #[default]
  Float,
  Base64,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateEmbeddingRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct CreateEmbeddingRequest {
  /// ID of the model to use. You can use the
  /// [List models](https://platform.openai.com/docs/api-reference/models/list)
  /// API to see all of your available models, or see our
  /// [Model overview](https://platform.openai.com/docs/models/overview)
  /// for descriptions of them.
  pub model: String,

  ///  Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub input: Option<EmbeddingInput>,

  /// The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/). Defaults to float
  #[serde(skip_serializing_if = "Option::is_none")]
  pub encoding_format: Option<EncodingFormat>,

  /// A unique identifier representing your end-user, which will help OpenAI
  ///  to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<CreateEmbeddingRequest> for async_openai::types::CreateEmbeddingRequest {
  fn from(req: CreateEmbeddingRequest) -> Self {
    Self {
      model: req.model,
      input: req
        .input
        .map(|input| match input {
          EmbeddingInput::String(s) => async_openai::types::EmbeddingInput::String(s),
          EmbeddingInput::StringArray(arr) => async_openai::types::EmbeddingInput::StringArray(arr),
          EmbeddingInput::IntegerArray(arr) => {
            async_openai::types::EmbeddingInput::IntegerArray(arr)
          }
          EmbeddingInput::ArrayOfIntegerArray(arr) => {
            async_openai::types::EmbeddingInput::ArrayOfIntegerArray(arr)
          }
        })
        .unwrap_or_else(|| async_openai::types::EmbeddingInput::String("".to_string())),
      encoding_format: req.encoding_format.map(|format| match format {
        EncodingFormat::Float => async_openai::types::EncodingFormat::Float,
        EncodingFormat::Base64 => async_openai::types::EncodingFormat::Base64,
      }),
      user: req.user,
    }
  }
}

/// Represents an embedding vector returned by embedding endpoint.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Embedding {
  /// The index of the embedding in the list of embeddings.
  pub index: u32,
  /// The object type, which is always "embedding".
  pub object: String,
  /// The embedding vector, which is a list of floats. The length of vector
  /// depends on the model as listed in the [embedding guide](https://platform.openai.com/docs/guides/embeddings).
  pub embedding: Vec<f32>,
}

#[cfg(feature = "ssr")]
impl From<Embedding> for async_openai::types::Embedding {
  fn from(embedding: Embedding) -> Self {
    Self {
      index: embedding.index,
      object: embedding.object,
      embedding: embedding.embedding,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::Embedding> for Embedding {
  fn from(embedding: async_openai::types::Embedding) -> Self {
    Self {
      index: embedding.index,
      object: embedding.object,
      embedding: embedding.embedding,
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EmbeddingUsage {
  /// The number of tokens used by the prompt.
  pub prompt_tokens: u32,
  /// The total number of tokens used by the request.
  pub total_tokens: u32,
}

#[cfg(feature = "ssr")]
impl From<EmbeddingUsage> for async_openai::types::EmbeddingUsage {
  fn from(usage: EmbeddingUsage) -> Self {
    Self {
      prompt_tokens: usage.prompt_tokens,
      total_tokens: usage.total_tokens,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::EmbeddingUsage> for EmbeddingUsage {
  fn from(usage: async_openai::types::EmbeddingUsage) -> Self {
    Self {
      prompt_tokens: usage.prompt_tokens,
      total_tokens: usage.total_tokens,
    }
  }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateEmbeddingResponse {
  pub object: String,
  /// The name of the model used to generate the embedding.
  pub model: String,
  /// The list of embeddings generated by the model.
  pub data: Vec<Embedding>,
  /// The usage information for the request.
  pub usage: EmbeddingUsage,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::CreateEmbeddingResponse> for CreateEmbeddingResponse {
  fn from(res: async_openai::types::CreateEmbeddingResponse) -> Self {
    Self {
      object: res.object,
      model: res.model,
      data: res.data.into_iter().map(|v| v.into()).collect(),
      usage: EmbeddingUsage::from(res.usage),
    }
  }
}
