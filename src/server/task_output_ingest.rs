//! HTTP adapter for process-local worker output ingestion.

mod http_error;
mod request;
mod service;
#[cfg(test)]
mod tests;

use super::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use request::TaskOutputPayload;

/// Accept progress without reopening terminal tasks.
///
/// # Errors
/// Returns 404 for missing tasks and 409 for tasks that reject progress.
pub(super) async fn agent_task_output(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    Json(payload): Json<TaskOutputPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    service::ingest(&state.knative_tasks, &state.bus, &task_id, &payload)
        .await
        .map_err(|error| http_error::response(error, &task_id))?;
    Ok(Json(serde_json::json!({
        "task_id": task_id,
        "status": "received",
    })))
}
