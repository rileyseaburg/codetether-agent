//! POST /v1/worker/tasks/release — release a completed/failed task.

use crate::server::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ReleaseRequest {
    pub task_id: String,
    pub status: String,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub diagnostics: Option<serde_json::Value>,
}

/// Release a task after processing is complete (or failed).
///
/// Updates the task status in the queue and publishes a bus event
/// so other components (dashboard, audit) are notified.
pub async fn worker_task_release(
    State(state): State<AppState>,
    axum::extract::Json(req): axum::extract::Json<ReleaseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let new_status = if req.status == "completed" || req.status == "success" {
        "completed"
    } else {
        "failed"
    };

    let updated = state
        .knative_tasks
        .update_status(&req.task_id, new_status)
        .await;

    if !updated {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Task {} not found", req.task_id),
        ));
    }

    // Notify bus subscribers about the task completion.
    let handle = state.bus.handle("worker_task_release");
    handle.send(
        format!("task.{}", req.task_id),
        crate::bus::BusMessage::TaskUpdate {
            task_id: req.task_id.clone(),
            state: crate::a2a::types::TaskState::Completed,
            message: req
                .result
                .clone()
                .or_else(|| req.error.clone().map(|e| format!("Error: {e}"))),
        },
    );

    tracing::info!(
        task_id = %req.task_id,
        status = new_status,
        "Task released by worker"
    );

    Ok(Json(serde_json::json!({
        "task_id": req.task_id,
        "status": new_status,
    })))
}
