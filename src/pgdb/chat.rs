use chrono::prelude::*;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, Either, PgPool};
use struct_convert::Convert;
use uuid::Uuid;

use crate::{
  models::{
    Chat as AppChat, ChatCompletionRequestAssistantMessage,
    ChatCompletionRequestAssistantMessageBuilder, ChatCompletionRequestFunctionMessage,
    ChatCompletionRequestSystemMessage, ChatCompletionRequestToolMessage,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
    ChatLog as AppChatLog, ChatMessage, Role, SavedMessage,
  },
  Result,
};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, Convert)]
#[convert(from = "AppChat")]
pub struct Chat {
  pub id: Uuid,
  pub title: Option<String>,
  pub user_id: Uuid,
  #[convert_field(rename = "email")]
  pub user_email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<Chat> for AppChat {
  fn from(value: Chat) -> Self {
    AppChat {
      id: value.id,
      title: value.title,
      user_id: value.user_id,
      email: value.user_email,
      created_at: value.created_at,
      updated_at: value.updated_at,
      ..Default::default()
    }
  }
}

impl From<(Chat, Vec<Message>, Vec<Variable>, Vec<Log>)> for AppChat {
  fn from(value: (Chat, Vec<Message>, Vec<Variable>, Vec<Log>)) -> Self {
    let (chat, messages, variables, logs) = value;

    AppChat {
      id: chat.id,
      title: chat.title,
      user_id: chat.user_id,
      email: chat.user_email,
      created_at: chat.created_at,
      updated_at: chat.updated_at,
      messages: messages.into_iter().map(|m| m.into()).collect(),
      logs: logs.into_iter().map(|l| l.into()).collect(),
      variables: variables.into_iter().map(|v| (v.key, v.value)).collect(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Log {
  pub id: Uuid,
  pub chat_id: Uuid,
  pub user_id: Uuid,
  pub title: String,
  pub content: Option<String>,
  pub created_at: DateTime<Utc>,
}

impl From<Log> for AppChatLog {
  fn from(value: Log) -> Self {
    AppChatLog {
      title: value.title,
      content: value.content,
      user_id: value.user_id,
      color: None,
    }
  }
}

impl From<AppChatLog> for Log {
  fn from(value: AppChatLog) -> Self {
    Self {
      id: Uuid::new_v4(),
      chat_id: Uuid::new_v4(),
      user_id: value.user_id,
      title: value.title,
      content: value.content,
      created_at: Utc::now(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Message {
  pub id: Uuid,
  pub chat_id: Uuid,
  pub user_id: Uuid,
  pub content: Option<String>,
  pub name: Option<String>,
  pub tool_calls: Json<Option<serde_json::Value>>,
  pub temporary: bool,
  pub role: String,
  pub tool_call_id: Option<String>,
  pub created_at: DateTime<Utc>,
}

impl From<Message> for SavedMessage {
  fn from(value: Message) -> Self {
    match value.role.to_lowercase().as_str() {
      "system" => SavedMessage {
        msg: ChatMessage::System(ChatCompletionRequestSystemMessage {
          role: Role::System,
          content: value.content.unwrap_or_default(),
          name: value.name,
        }),
        temporary: value.temporary,
      },
      "user" => SavedMessage {
        msg: ChatMessage::User(ChatCompletionRequestUserMessage {
          content: ChatCompletionRequestUserMessageContent::Text(value.content.unwrap_or_default()),
          role: Role::User,
          name: value.name,
        }),
        temporary: value.temporary,
      },
      "assistant" => {
        let mut msg = ChatCompletionRequestAssistantMessageBuilder::default();
        if let Some(content) = value.content.as_ref() {
          msg.content(content);
        }
        #[allow(deprecated)]
        SavedMessage {
          msg: ChatMessage::Assistant(ChatCompletionRequestAssistantMessage {
            content: value.content.clone(),
            role: Role::Assistant,
            name: value.name,
            tool_calls: value
              .tool_calls
              .0
              .map(|v| serde_json::from_value(v).unwrap()),
          }),
          temporary: value.temporary,
        }
      }
      "tool" => SavedMessage {
        msg: ChatMessage::Tool(ChatCompletionRequestToolMessage {
          role: Role::Tool,
          content: value.content.unwrap_or_default(),
          tool_call_id: value.tool_call_id.unwrap_or_default(),
        }),
        temporary: value.temporary,
      },
      "function" => SavedMessage {
        msg: ChatMessage::Function(ChatCompletionRequestFunctionMessage {
          role: Role::Function,
          content: value.content,
          name: value.name.unwrap_or_default(),
        }),
        temporary: value.temporary,
      },
      _ => unreachable!(),
    }
  }
}

impl From<SavedMessage> for Message {
  fn from(value: SavedMessage) -> Self {
    let (content, name, tool_calls, role, tool_call_id) = match value.msg {
      ChatMessage::System(msg) => (Some(msg.content), msg.name, None, "system", None),
      ChatMessage::User(msg) => match msg.content {
        ChatCompletionRequestUserMessageContent::Text(content) => {
          (Some(content), msg.name, None, "user", None)
        }
        _ => (None, msg.name, None, "user", None),
      },
      ChatMessage::Assistant(msg) => {
        let tool_calls = msg.tool_calls.map(|v| serde_json::to_value(v).unwrap());
        (msg.content, msg.name, tool_calls, "assistant", None)
      }
      ChatMessage::Tool(msg) => (
        Some(msg.content),
        None,
        None,
        "tool",
        Some(msg.tool_call_id),
      ),
      ChatMessage::Function(msg) => (msg.content, Some(msg.name), None, "function", None),
    };
    Self {
      id: Uuid::new_v4(),
      chat_id: Uuid::new_v4(),
      user_id: Uuid::new_v4(),
      content,
      name,
      tool_calls: Json(tool_calls),
      temporary: value.temporary,
      role: role.to_string(),
      tool_call_id,
      created_at: Utc::now(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Variable {
  pub id: Uuid,
  pub chat_id: Uuid,
  pub user_id: Uuid,
  pub key: String,
  pub value: String,
  pub created_at: DateTime<Utc>,
}

impl Chat {
  pub async fn find_for_user(id: Uuid, pool: &PgPool) -> Result<Vec<AppChat>> {
    let mut chats = sqlx::query_file_as!(Chat, "queries/chats/chat_list.sql", id).fetch_many(pool);

    let mut res = vec![];
    while let Some(chat) = chats.try_next().await? {
      if let Either::Right(chat) = chat {
        res.push(chat.into());
      }
    }

    Ok(res)
  }

  pub async fn get(id: Uuid, pool: &PgPool) -> Result<AppChat> {
    let chat = sqlx::query_file_as!(Chat, "queries/chats/chat_get.sql", id).fetch_one(pool);
    let messages =
      sqlx::query_file_as!(Message, "queries/messages/get_for_chat.sql", id).fetch_all(pool);

    let variables =
      sqlx::query_file_as!(Variable, "queries/variables/get_for_chat.sql", id).fetch_all(pool);

    let logs = sqlx::query_file_as!(Log, "queries/logs/get_for_chat.sql", id).fetch_all(pool);

    let parts = futures::try_join!(chat, messages, variables, logs)?;
    Ok(parts.into())
  }

  pub async fn create(id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<AppChat> {
    let chat = sqlx::query_file_as!(Chat, "queries/chats/chat_create.sql", id, user_id)
      .fetch_one(pool)
      .await?;

    Ok(chat.into())
  }

  pub async fn delete(id: Uuid, pool: &PgPool) -> Result<()> {
    sqlx::query_file!("queries/chats/chat_delete.sql", id)
      .execute(pool)
      .await?;
    Ok(())
  }

  pub async fn update_title(id: Uuid, title: String, pool: &PgPool) -> Result<AppChat> {
    let chat = sqlx::query_file_as!(Chat, "queries/chats/title_update.sql", title, id)
      .fetch_one(pool)
      .await?;
    Ok(chat.into())
  }
}

impl Log {
  pub async fn create(
    chat_id: Uuid,
    user_id: Uuid,
    title: String,
    content: Option<String>,
    pool: &PgPool,
  ) -> Result<AppChatLog> {
    let log = sqlx::query_file_as!(
      Log,
      "queries/logs/add_new.sql",
      chat_id,
      user_id,
      title,
      content
    )
    .fetch_one(pool)
    .await?;
    Ok(log.into())
  }
}
