//! Worker release HTTP extraction; lifecycle ownership is shared.
use crate::server::AppState;
use axum::{Json, extract::State, http::StatusCode};
pub(crate) use codetether_task_api::release::ReleaseRequest;
/// Extract worker completion and delegate its lifecycle transition.
pub async fn worker_task_release(
    State(state): State<AppState>,
    Json(req): Json<ReleaseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    codetether_task_api::release::respond(&state.knative_tasks, &state.bus, &req).await
}
