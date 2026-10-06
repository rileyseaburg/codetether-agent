//! POST /v1/worker/tasks/release — release a completed/failed task.

#[path = "worker_task_release/http_error.rs"]
mod http_error;
#[path = "worker_task_release/outcome.rs"]
mod outcome;
#[path = "worker_task_release/request.rs"]
mod request;
#[path = "worker_task_release/service.rs"]
mod service;
#[cfg(test)]
#[path = "worker_task_release/tests.rs"]
mod tests;

use crate::server::AppState;
use axum::{Json, extract::State, http::StatusCode};
pub(crate) use request::ReleaseRequest;

/// Release a task after processing is complete (or failed).
///
/// Updates the task status in the queue and publishes a bus event
/// so other components (dashboard, audit) are notified.
///
/// # Errors
/// Returns HTTP 404 for missing tasks and 409 for inactive/terminal tasks.
pub async fn worker_task_release(
    State(state): State<AppState>,
    Json(req): Json<ReleaseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let status = service::release(&state.knative_tasks, &state.bus, &req)
        .await
        .map_err(|reason| http_error::from_release(&req.task_id, reason))?;
    Ok(Json(serde_json::json!({
        "task_id": req.task_id,
        "status": status,
    })))
}
