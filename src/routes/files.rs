use cfg_if::cfg_if;
use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use uuid::Uuid;

use crate::models::UploadedFile;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use crate::Result;
    use crate::app::{auth,app_state};
    use crate::server::workspace;
    use std::path::PathBuf;
    use std::str::FromStr;
  }
}

#[server(GetFiles, "/api")]
pub async fn get_files(chat_id: Uuid) -> Result<Vec<UploadedFile>, ServerFnError> {
  let auth = auth()?;
  let app_state = app_state()?;
  match auth.current_user {
    Some(_user) => {
      let mut upload_base = app_state.upload_store.clone();
      upload_base.push(chat_id.to_string());
      let mut dir_entries = tokio::fs::read_dir(upload_base.as_path()).await?;

      let mut files = vec![];
      while let Some(entry) = dir_entries
        .next_entry()
        .await
        .map_err(ServerFnError::WrappedServerError)?
      {
        let path = entry.path();
        files.push(UploadedFile {
          file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
          mime_type: mime_guess::from_path(&path)
            .first_or_octet_stream()
            .to_string(),
          workspace: chat_id.to_string(),
        })
      }
      Ok(files)
    }
    None => Err(ServerFnError::ServerError("Not authenticated.".into())),
  }
}

#[server(input = MultipartFormData)]
#[tracing::instrument(skip(data))]
pub async fn upload_files(data: MultipartData) -> Result<usize, ServerFnError> {
  let auth = auth()?;
  let app_state = app_state()?;
  match auth.current_user {
    Some(_user) => {
      let mut data = data.into_inner().unwrap();
      let mut workspace_dir = PathBuf::new();
      let mut count = 0;
      let mut collected_fields = vec![];
      let mut found_chat_id = false;
      while let Ok(Some(field)) = data.next_field().await {
        if field.name() == Some("chat_id") {
          found_chat_id = true;
          let chat_id = field.text().await?;
          workspace_dir = app_state.upload_store.join(chat_id);
          if !tokio::fs::try_exists(&workspace_dir).await? {
            tokio::fs::create_dir_all(&workspace_dir).await?;
          }
          break;
        }

        collected_fields.push(field);
      }

      if !found_chat_id {
        return Err(ServerFnError::ServerError("No chat_id field found.".into()));
      }

      for field in collected_fields {
        let file_name = if let Some(file_name) = field.file_name() {
          PathBuf::from_str(file_name).unwrap() // this is infallible
        } else {
          continue;
        };

        workspace::stream_to_file(&workspace_dir, &file_name, field).await?;
        count += 1;
      }
      while let Ok(Some(field)) = data.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
          PathBuf::from_str(file_name).unwrap() // this is infallible
        } else {
          continue;
        };

        workspace::stream_to_file(&workspace_dir, &file_name, field).await?;
        count += 1;
      }
      Ok(count)
    }
    None => Err(ServerFnError::ServerError("Not authenticated.".into())),
  }
}
