use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFile {
  pub workspace: String,
  pub file_name: String,
  pub mime_type: String,
}
