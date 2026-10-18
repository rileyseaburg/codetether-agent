//! Worker release HTTP extraction; lifecycle ownership is shared.
use crate::http::HttpState;
use crate::release::ReleaseRequest;
use axum::{Json, extract::State, http::StatusCode};
/// Extract worker completion and delegate its lifecycle transition.
/// # Arguments
/// Receives application state and the decoded completion request.
/// # Returns
/// The shared release handler's JSON response.
/// # Errors
/// Returns the shared release handler's HTTP status and message.
/// # Examples
/// ```no_run
/// # fn routes<S: codetether_task_api::http::HttpState>() -> axum::Router<S> {
/// axum::Router::new().route("/release", axum::routing::post(
///     codetether_task_api::worker_http::worker_task_release::<S>,
/// ))
/// # }
/// ```
pub async fn worker_task_release<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<ReleaseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    crate::release::respond(state.queue(), state.bus(), &req).await
}
