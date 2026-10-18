//! Worker claim HTTP extraction and delegation.
use crate::a2a::claim::TaskClaimResponse;
use crate::server::AppState;
use axum::{Json, extract::State, http::StatusCode};
use codetether_task_api::worker_claim::ClaimRequest;
/// Extract a claim request and delegate its atomic queue transition.
pub async fn worker_task_claim(
    State(state): State<AppState>,
    Json(req): Json<ClaimRequest>,
) -> Result<Json<TaskClaimResponse>, (StatusCode, String)> {
    codetether_task_api::worker_claim::claim(&state.inner.knative_tasks, &req).await
}
