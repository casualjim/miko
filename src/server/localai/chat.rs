use std::time::Duration;

use async_openai::types::CreateChatCompletionRequest;
use axum::{
  extract::State,
  response::{
    sse::{Event, KeepAlive},
    IntoResponse, Sse,
  },
  routing::{get, post},
  Json,
};
use futures::StreamExt;

use crate::{app::state::AppState, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/completions", post(completions))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state))]
async fn completions(
  State(app_state): State<AppState>,
  Json(params): Json<CreateChatCompletionRequest>,
) -> Result<impl IntoResponse> {
  if params.stream.unwrap_or_default() {
    let mut result = app_state
      .openai_client()
      .chat()
      .create_stream(params)
      .await?;
    let (tx, rx) = futures::channel::mpsc::unbounded();
    tokio::spawn(async move {
      while let Some(event) = result.next().await {
        if let Ok(event) = event {
          tx.unbounded_send(Event::default().json_data(event))
            .unwrap();
        }
      }
    });
    return Ok(
      Sse::new(rx)
        .keep_alive(
          KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive-text"),
        )
        .into_response(),
    );
  }
  let result = app_state.openai_client().chat().create(params).await?;
  Ok(Json(result).into_response())
}
