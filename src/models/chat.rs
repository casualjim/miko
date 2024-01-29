use std::collections::HashMap;

use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ChatError {
  /// Error from client side validation
  /// or when builder fails to build request before making API call
  #[error("invalid Builder: {0}")]
  InvalidArgument(String),
}

/// OpenAI API returns error object on failure
#[derive(Debug, Deserialize)]
pub struct ApiError {
  pub message: String,
  pub r#type: Option<String>,
  pub param: Option<serde_json::Value>,
  pub code: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatInfo {
  pub id: Uuid,
  pub name: Option<String>,
}

impl From<Chat> for ChatInfo {
  fn from(chat: Chat) -> Self {
    Self {
      id: chat.id,
      name: chat.title.or_else(|| Some("New Session".to_string())),
    }
  }
}

impl From<&Chat> for ChatInfo {
  fn from(chat: &Chat) -> Self {
    Self {
      id: chat.id,
      name: chat
        .title
        .clone()
        .or_else(|| Some("New Session".to_string())),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditChat {
  pub id: Uuid,
  pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubmittedGoal {
  pub chat_id: Uuid,
  pub goal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Chat {
  pub id: Uuid,
  pub title: Option<String>,
  pub user_id: Uuid,
  pub email: String,
  pub messages: Vec<SavedMessage>,
  pub logs: Vec<ChatLog>,
  pub variables: HashMap<String, String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedMessage {
  pub msg: ChatMessage,
  pub temporary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatMessage {
  System(ChatCompletionRequestSystemMessage),
  User(ChatCompletionRequestUserMessage),
  Assistant(ChatCompletionRequestAssistantMessage),
  Tool(ChatCompletionRequestToolMessage),
  Function(ChatCompletionRequestFunctionMessage),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestSystemMessageBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ChatCompletionRequestSystemMessage {
  /// The contents of the system message.
  pub content: String,
  /// The role of the messages author, in this case `system`.
  #[builder(default = "Role::System")]
  pub role: Role,
  /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartTextBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ChatCompletionRequestMessageContentPartText {
  #[builder(default = "\"text\".into()")]
  pub r#type: String,
  pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageUrlDetail {
  #[default]
  Auto,
  Low,
  High,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ImageUrlBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ImageUrl {
  /// Either a URL of the image or the base64 encoded image data.
  pub url: String,
  /// Specifies the detail level of the image. Learn more in the [Vision guide](https://platform.openai.com/docs/guides/vision/low-or-high-fidelity-image-understanding).
  pub detail: ImageUrlDetail,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartImageBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ChatCompletionRequestMessageContentPartImage {
  #[builder(default = "\"image_url\".into()")]
  pub r#type: String,
  pub image_url: ImageUrl,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPart {
  Text(ChatCompletionRequestMessageContentPartText),
  Image(ChatCompletionRequestMessageContentPartImage),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
  /// The text contents of the message.
  Text(String),
  ///  An array of content parts with a defined type, each can be of type `text` or `image_url`
  /// when passing in images. You can pass multiple images by adding multiple `image_url` content parts.
  ///  Image input is only supported when using the `gpt-4-visual-preview` model.
  Array(Vec<ChatCompletionRequestMessageContentPart>),
}

impl Default for ChatCompletionRequestUserMessageContent {
  fn default() -> Self {
    Self::Text(String::new())
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestUserMessageBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ChatCompletionRequestUserMessage {
  /// The contents of the user message.
  pub content: ChatCompletionRequestUserMessageContent,
  /// The role of the messages author, in this case `user`.
  #[builder(default = "Role::User")]
  pub role: Role,
  /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestAssistantMessageBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ChatCompletionRequestAssistantMessage {
  /// The contents of the assistant message.
  pub content: Option<String>,
  /// The role of the messages author, in this case `assistant`.
  #[builder(default = "Role::Assistant")]
  pub role: Role,
  /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
}

/// Tool message
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestToolMessageBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ChatCompletionRequestToolMessage {
  /// The role of the messages author, in this case `tool`.
  #[builder(default = "Role::Tool")]
  pub role: Role,
  /// The contents of the tool message.
  pub content: String,
  pub tool_call_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestFunctionMessageBuilder")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ChatError"))]
pub struct ChatCompletionRequestFunctionMessage {
  /// The role of the messages author, in this case `function`.
  #[builder(default = "Role::Function")]
  pub role: Role,
  /// The return value from the function call, to return to the model.
  pub content: Option<String>,
  /// The name of the function to call.
  pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCall {
  /// The ID of the tool call.
  pub id: String,
  /// The type of the tool. Currently, only `function` is supported.
  pub r#type: ChatCompletionToolType,
  /// The function that the model called.
  pub function: FunctionCall,
}

/// The name and arguments of a function that should be called, as generated by the model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FunctionCall {
  /// The name of the function to call.
  pub name: String,
  /// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
  pub arguments: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct ChatLog {
  pub title: String,
  pub content: Option<String>,
  pub user: String,
  pub color: Option<String>,
  pub created_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolType {
  #[default]
  Function,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  System,
  #[default]
  User,
  Assistant,
  Tool,
  Function,
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use sqlx::PgPool;
    use crate::pgdb::{Chat as SqlChat, Log as SqlChatLog};
    use crate::Result;
    impl Chat {
      pub async fn find_for_user(id: Uuid, pool: &PgPool) -> Result<Vec<Chat>> {
        SqlChat::find_for_user(id, pool).await
      }

      pub async fn get(id: Uuid, pool: &PgPool) -> Result<Chat> {
        SqlChat::get(id, pool).await
      }

      pub async fn create(id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<Chat> {
        SqlChat::create(id, user_id, pool).await
      }

      pub async fn delete(id: Uuid, pool: &PgPool) -> Result<()> {
        SqlChat::delete(id, pool).await
      }

      pub async fn update_title(id: Uuid, title: String, pool: &PgPool) -> Result<Chat> {
        SqlChat::update_title(id, title, pool).await
      }
    }

    impl ChatLog {
      pub async fn create(chat_id: Uuid, user: String, title: String, content: Option<String>, pool: &PgPool) -> Result<ChatLog> {
        SqlChatLog::create(chat_id, user, title, content, pool).await
      }

      pub async fn list(chat_id: Uuid, pool: &PgPool) -> Result<Vec<ChatLog>> {
        SqlChatLog::list(chat_id, pool).await
      }
    }
  }
}
