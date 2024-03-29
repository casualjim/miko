pub mod audio;
mod chat;
pub mod embeddings;
mod files;
pub mod fine_tuning;
pub mod images;
pub mod moderation;
mod user;

pub use chat::*;
use derive_builder::UninitializedFieldError;
pub use files::UploadedFile;
pub use user::User;

impl From<UninitializedFieldError> for ChatError {
  fn from(value: UninitializedFieldError) -> Self {
    ChatError::InvalidArgument(value.to_string())
  }
}
#[derive(Debug, Clone, Default)]
pub struct CurrentUser(pub Option<User>);

impl CurrentUser {
  pub fn is_authenticated(&self) -> bool {
    self.0.is_some()
  }

  pub fn picture(&self) -> Option<String> {
    self.0.as_ref().and_then(|user| user.picture.clone())
  }

  pub fn name(&self) -> String {
    if let Some(user) = self.0.as_ref() {
      user.name.clone()
    } else {
      "Guest".to_string()
    }
  }

  pub fn name_opt(&self) -> Option<String> {
    if let Some(user) = self.0.as_ref() {
      Some(user.name.clone())
    } else {
      None
    }
  }

  pub fn name_or<I: Into<String>>(&self, default_value: I) -> String {
    if let Some(user) = self.0.as_ref() {
      user.name.clone()
    } else {
      default_value.into()
    }
  }

  pub fn email(&self) -> Option<String> {
    self.0.as_ref().map(|v| v.email.clone())
  }
}
