use axum::{extract::State, response::IntoResponse, routing::get, Json};

use crate::{app::state::AppState, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/models", get(list_models))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state))]
async fn list_models(State(app_state): State<AppState>) -> Result<impl IntoResponse> {
  let models = app_state.openai_client().models().list().await?;
  Ok(Json(models))
}
