mod chat;
mod files;
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

  pub fn email(&self) -> String {
    self
      .0
      .as_ref()
      .map(|user| user.email.clone())
      .unwrap_or_default()
  }
}
