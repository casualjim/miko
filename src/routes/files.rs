use std::path::Path;

use cfg_if::cfg_if;
use leptos::*;

use crate::{app, models::UploadedFile};

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use crate::Result;
    use crate::app::{auth,app_state,pool};
  }
}

#[server(GetFiles, "/bff")]
pub async fn get_files() -> Result<Vec<UploadedFile>, ServerFnError> {
  let auth = auth()?;
  let app_state = app_state()?;
  match auth.current_user {
    Some(user) => {
      let mut upload_base = app_state.upload_store.clone();
      upload_base.push(user.id.to_string());
      let mut dir_entries = tokio::fs::read_dir(upload_base.as_path()).await?;

      let mut files = vec![];
      while let Some(entry) = dir_entries
        .next_entry()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
      {
        let path = entry.path();
        files.push(UploadedFile {
          path: path.to_string_lossy().to_string(),
        })
      }
      Ok(files)
    }
    None => Err(ServerFnError::ServerError("Not authenticated.".into())),
  }
}
