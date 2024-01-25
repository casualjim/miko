mod audio;
mod chat;
mod embeddings;
mod files;
mod fine_tuning;
mod images;
mod models;
mod moderations;
use axum::{extract::State, response::IntoResponse, routing::get, Json};

use crate::{app::state::AppState, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .nest("/audio", audio::routes(app_state.clone()))
    .nest("/chat", chat::routes(app_state.clone()))
    .nest("/embeddings", embeddings::routes(app_state.clone()))
    .nest("/fine_tuning", fine_tuning::routes(app_state.clone()))
    .nest("/files", files::routes(app_state.clone()))
    .nest("/images", images::routes(app_state.clone()))
    .nest("/models", models::routes(app_state.clone()))
    .nest("/moderations", moderations::routes(app_state.clone()))
    .with_state(app_state)
}
