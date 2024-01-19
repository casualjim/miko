mod contexts;
pub mod models;
mod routes;
#[cfg(feature = "ssr")] pub mod server;

use cfg_if::cfg_if;
pub mod app;
mod components;
pub mod error_template;
pub mod fileserv;
mod pages;

pub(crate) use contexts::*;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}

cfg_if! { if #[cfg(feature = "ssr")]{
  pub mod pgdb;

  use thiserror::Error;
  use http::StatusCode;
  use axum::response::IntoResponse;
  use axum::Json;
  use serde_json::json;
  use async_openai::error::OpenAIError;

  pub type Result<T, E = Error> = core::result::Result<T, E>;

  #[derive(Debug, Error)]
  pub enum Error {
    #[error("unknown error")]
    Unknown,
    #[error("Corrupt Session")]
    CorruptSession,
    #[error("User not found")]
    UserNotFound,
    #[error("User has no credentials")]
    UserHasNoCredentials,
    #[error("User is not authenticated")]
    UserNotAuthenticated,
    #[error("database: {0}")]
    Pgx(#[from] sqlx::Error),
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("openai: {0}")]
    OpenAI(#[from] OpenAIError),
    #[error("invalid args: {0}")]
    InvalidArgument(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    // #[error("uninitialized field: {0}")]
    // UninitializedField(#[from] UninitializedFieldError),
  }

  impl Error {
    pub fn status_code(&self) -> StatusCode {
      match self {
        Error::CorruptSession => StatusCode::INTERNAL_SERVER_ERROR,
        Error::UserNotFound => StatusCode::NOT_FOUND,
        Error::UserHasNoCredentials => StatusCode::FORBIDDEN,
        Error::UserNotAuthenticated => StatusCode::UNAUTHORIZED,
        Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        Error::Pgx(sqlx::Error::Database(db_err)) if db_err.code() == Some("23505".into()) => {
          StatusCode::CONFLICT
        }
        Error::Pgx(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
        Error::Pgx(_e) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::Serde(_e) => StatusCode::BAD_REQUEST,
        Error::OpenAI(_e) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::InvalidArgument(_e) => StatusCode::BAD_REQUEST,
        Error::Io(_e) => StatusCode::INTERNAL_SERVER_ERROR,
        // Error::UninitializedField(_e) => StatusCode::BAD_REQUEST,
      }
    }
  }

  impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
      (
        self.status_code(),
        Json(json!({"message": self.to_string()})),
      )
        .into_response()
    }
  }

}}
