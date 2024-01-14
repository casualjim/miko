use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

use crate::models::Chat;

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
  }
}
#[server(GetChats, "/bff")]
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

#[server(GenerateTitle, "/bff")]
pub async fn generate_title(id: Uuid, prompt: String) -> Result<String, ServerFnError> {
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
    .map_err(|e| ServerFnError::ServerError(format!("OpenAI error: {}", e)))?;

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
