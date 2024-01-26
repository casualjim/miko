use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::ChatError;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(untagged)]
pub enum NEpochs {
  NEpochs(u8),
  #[default]
  #[serde(rename = "auto")]
  Auto,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::NEpochs> for NEpochs {
  fn from(req: async_openai::types::NEpochs) -> Self {
    match req {
      async_openai::types::NEpochs::NEpochs(n) => Self::NEpochs(n),
      async_openai::types::NEpochs::Auto => Self::Auto,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<NEpochs> for async_openai::types::NEpochs {
  fn from(req: NEpochs) -> Self {
    match req {
      NEpochs::NEpochs(n) => Self::NEpochs(n),
      NEpochs::Auto => Self::Auto,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Hyperparameters {
  /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
  pub n_epochs: NEpochs,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::Hyperparameters> for Hyperparameters {
  fn from(req: async_openai::types::Hyperparameters) -> Self {
    Self {
      n_epochs: match req.n_epochs {
        async_openai::types::NEpochs::NEpochs(n) => NEpochs::NEpochs(n),
        async_openai::types::NEpochs::Auto => NEpochs::Auto,
      },
    }
  }
}

#[cfg(feature = "ssr")]
impl From<Hyperparameters> for async_openai::types::Hyperparameters {
  fn from(req: Hyperparameters) -> Self {
    Self {
      n_epochs: match req.n_epochs {
        NEpochs::NEpochs(n) => async_openai::types::NEpochs::NEpochs(n),
        NEpochs::Auto => async_openai::types::NEpochs::Auto,
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateFineTuningJobRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct CreateFineTuningJobRequest {
  /// The name of the model to fine-tune. You can select one of the
  /// [supported models](https://platform.openai.com/docs/guides/fine-tuning/what-models-can-be-fine-tuned).
  pub model: String,

  /// The ID of an uploaded file that contains training data.
  ///
  /// See [upload file](https://platform.openai.com/docs/api-reference/files/upload) for how to upload a file.
  ///
  /// Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.
  ///
  /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
  pub training_file: String,

  /// The hyperparameters used for the fine-tuning job.
  pub hyperparameters: Option<Hyperparameters>,

  /// A string of up to 18 characters that will be added to your fine-tuned model name.
  ///
  /// For example, a `suffix` of "custom-model-name" would produce a model name
  /// like `ft:gpt-3.5-turbo:openai:custom-model-name:7p4lURel`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub suffix: Option<String>, // default: null, minLength:1, maxLength:40

  /// The ID of an uploaded file that contains validation data.
  ///
  /// If you provide this file, the data is used to generate validation
  /// metrics periodically during fine-tuning. These metrics can be viewed in
  /// the fine-tuning results file.
  /// The same data should not be present in both train and validation files.
  ///
  /// Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.
  ///
  /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub validation_file: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::CreateFineTuningJobRequest> for CreateFineTuningJobRequest {
  fn from(req: async_openai::types::CreateFineTuningJobRequest) -> Self {
    Self {
      model: req.model,
      training_file: req.training_file,
      hyperparameters: req.hyperparameters.map(Into::into),
      suffix: req.suffix,
      validation_file: req.validation_file,
    }
  }
}

#[cfg(feature = "ssr")]
impl From<CreateFineTuningJobRequest> for async_openai::types::CreateFineTuningJobRequest {
  fn from(req: CreateFineTuningJobRequest) -> Self {
    Self {
      model: req.model,
      training_file: req.training_file,
      hyperparameters: req.hyperparameters.map(Into::into),
      suffix: req.suffix,
      validation_file: req.validation_file,
    }
  }
}
