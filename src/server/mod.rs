use crate::app::state::AppState;

pub mod localai;
pub mod workspace;

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .nest("/workspace", workspace::routes(app_state.clone()))
    .with_state(app_state)
}
