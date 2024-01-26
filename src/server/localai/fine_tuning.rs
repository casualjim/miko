use async_openai::types::{
  FineTuningJob, ListFineTuningJobEventsResponse, ListPaginatedFineTuningJobsResponse,
};
use axum::{
  extract::{Path, Query, State},
  routing::{get, post},
  Json,
};
use serde::{Deserialize, Serialize};

use crate::{app::state::AppState, models::fine_tuning::CreateFineTuningJobRequest, Result};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/jobs", get(list_jobs).post(create_job))
    .route("/jobs/:fine_tuning_job_id", get(get_job))
    .route("/jobs/:fine_tuning_job_id/events", get(list_job_events))
    .route("/jobs/:fine_tuning_job_id/cancel", post(cancel_job))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state))]
async fn create_job(
  State(app_state): State<AppState>,
  Json(request): Json<CreateFineTuningJobRequest>,
) -> Result<Json<FineTuningJob>> {
  app_state
    .openai_client()
    .fine_tuning()
    .create(request.into())
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[derive(Debug, Serialize, Deserialize)]
struct ListJobsQuery {
  after: Option<String>,
  limit: Option<usize>,
}

#[tracing::instrument(skip(app_state))]
async fn list_jobs(
  State(app_state): State<AppState>,
  Query(params): Query<ListJobsQuery>,
) -> Result<Json<ListPaginatedFineTuningJobsResponse>> {
  app_state
    .openai_client()
    .fine_tuning()
    .list_paginated(&params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[tracing::instrument(skip(app_state))]
async fn get_job(
  State(app_state): State<AppState>,
  Path(fine_tuning_job_id): Path<String>,
) -> Result<Json<FineTuningJob>> {
  app_state
    .openai_client()
    .fine_tuning()
    .retrieve(&fine_tuning_job_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[tracing::instrument(skip(app_state))]
async fn list_job_events(
  State(app_state): State<AppState>,
  Path(fine_tuning_job_id): Path<String>,
  Query(params): Query<ListJobsQuery>,
) -> Result<Json<ListFineTuningJobEventsResponse>> {
  app_state
    .openai_client()
    .fine_tuning()
    .list_events(&fine_tuning_job_id, &params)
    .await
    .map_err(Into::into)
    .map(Into::into)
}

#[tracing::instrument(skip(app_state))]
async fn cancel_job(
  State(app_state): State<AppState>,
  Path(fine_tuning_job_id): Path<String>,
) -> Result<Json<FineTuningJob>> {
  app_state
    .openai_client()
    .fine_tuning()
    .cancel(&fine_tuning_job_id)
    .await
    .map_err(Into::into)
    .map(Into::into)
}
