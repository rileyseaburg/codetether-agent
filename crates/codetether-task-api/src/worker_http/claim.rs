//! Worker claim HTTP extraction and delegation.
use crate::http::HttpState;
use crate::worker_claim::ClaimRequest;
use axum::{Json, extract::State, http::StatusCode};
use codetether_a2a_worker_core::claim::TaskClaimResponse;
/// Extract a claim request and delegate its atomic queue transition.
/// # Arguments
/// Receives application state and the decoded claim request.
/// # Returns
/// The existing claim response after the queue transition.
/// # Errors
/// Returns the shared claim handler's HTTP status and message.
/// # Examples
/// ```no_run
/// # fn routes<S: codetether_task_api::http::HttpState>() -> axum::Router<S> {
/// axum::Router::new().route("/claim", axum::routing::post(
///     codetether_task_api::worker_http::worker_task_claim::<S>,
/// ))
/// # }
/// ```
pub async fn worker_task_claim<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<ClaimRequest>,
) -> Result<Json<TaskClaimResponse>, (StatusCode, String)> {
    crate::worker_claim::claim(state.queue(), &req).await
}
