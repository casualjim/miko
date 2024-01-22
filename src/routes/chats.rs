use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

use crate::models::{Chat, ChatLog};

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use async_openai::types::CreateChatCompletionRequest;
    use async_openai::types::{
      ChatCompletionRequestSystemMessageArgs,
      ChatCompletionRequestUserMessage,
      ChatCompletionRequestSystemMessage,
      ChatCompletionRequestMessage,
      ChatCompletionRequestUserMessageArgs,
    };

    use crate::Result;
    use crate::app::{auth,app_state,pool};
    use tracing::info;
  }
}
#[server(GetChats, "/api")]
pub async fn get_chats() -> Result<Vec<Chat>, ServerFnError> {
  let auth = auth()?;
  match auth.current_user {
    Some(user) => {
      let db = pool()?;
      let chats = Chat::find_for_user(user.id, &db).await?;
      Ok(chats)
    }
    None => Err(ServerFnError::ServerError("Not authenticated.".into())),
  }
}

#[server(CreateChat, "/api")]
pub async fn create_chat(id: Uuid) -> Result<(), ServerFnError> {
  let auth = auth()?;
  match auth.current_user {
    Some(user) => {
      let db = pool()?;
      let chat = Chat::create(id, user.id, &db).await?;
      leptos_axum::redirect(&format!("/chat/{}", chat.id));
      Ok(())
    }
    None => Err(ServerFnError::ServerError("Not authenticated.".into())),
  }
}

#[server(AddChatLog, "/api")]
pub async fn add_chat_log(
  chat_id: Uuid,
  title: String,
  content: Option<String>,
) -> Result<ChatLog, ServerFnError> {
  let auth = auth()?;
  match auth.current_user {
    Some(user) => {
      let db = pool()?;
      let chat = ChatLog::create(chat_id, user.id, title, content, &db).await?;
      Ok(chat)
    }
    None => Err(ServerFnError::ServerError("Not authenticated.".into())),
  }
}

#[server(DeleteChat, "/api")]
pub async fn delete_chat(id: Uuid) -> Result<(), ServerFnError> {
  let auth = auth()?;
  if !auth.is_authenticated() {
    return Err(ServerFnError::ServerError("Not authenticated.".into()));
  }

  info!("Deleting chat with id: {}", id);
  let db = pool()?;
  let app_state = app_state()?;
  tokio::fs::remove_dir_all(app_state.upload_store.join(id.to_string())).await?;
  Chat::delete(id, &db).await?;
  leptos_axum::redirect("/");
  Ok(())
}

#[server(UpdateChatTitle, "/api")]
pub async fn update_chat_title(id: Uuid, title: String) -> Result<(), ServerFnError> {
  let auth = auth()?;
  if !auth.is_authenticated() {
    return Err(ServerFnError::ServerError("Not authenticated.".into()));
  }

  let db = pool()?;
  Chat::update_title(id, title, &db).await?;
  Ok(())
}

#[server(GenerateTitle, "/api")]
pub async fn generate_title(_id: Uuid, prompt: String) -> Result<String, ServerFnError> {
  let auth = auth()?;
  if auth.current_user.is_none() {
    return Err(ServerFnError::ServerError("Not authenticated.".into()));
  }

  let app_state = app_state()?;
  let oai = app_state.openai_client();

  let sysprompt = make_sysprompt("Summarize the given prompt using max 4 words")?;
  let userprompt = make_userprompt(prompt)?;

  let response = oai
    .chat()
    .create(CreateChatCompletionRequest {
      messages: vec![
        ChatCompletionRequestMessage::System(sysprompt),
        ChatCompletionRequestMessage::User(userprompt),
      ],
      model: "gpt-3.5-turbo".into(),
      ..Default::default()
    })
    .await
    .map_err(ServerFnError::WrappedServerError)?;

  if response.choices.is_empty() {
    return Err(ServerFnError::ServerError(
      "No response from OpenAI.".into(),
    ));
  }

  response.choices[0]
    .message
    .content
    .clone()
    .ok_or_else(|| ServerFnError::ServerError("No content in OpenAI response.".into()))
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
    fn make_sysprompt<S: Into<String>>(prompt: S) -> Result<ChatCompletionRequestSystemMessage> {
      Ok(ChatCompletionRequestSystemMessageArgs::default().content(prompt).build()?)
    }

    fn make_userprompt<S: Into<String>>(prompt: S) -> Result<ChatCompletionRequestUserMessage> {
      Ok(ChatCompletionRequestUserMessageArgs::default().content(prompt.into()).build()?)
    }

  }
}
