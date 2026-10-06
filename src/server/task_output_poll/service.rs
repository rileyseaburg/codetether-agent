//! Lookup and presentation of the process-local completion receipt.

use crate::server::KnativeTaskQueue;
use axum::http::StatusCode;

/// Read a task receipt without mutating its lifecycle.
///
/// # Errors
/// Returns HTTP 404 when the queue has no task with the requested identity.
pub(super) async fn snapshot(
    tasks: &KnativeTaskQueue,
    task_id: &str,
) -> Result<serde_json::Value, (StatusCode, String)> {
    let task = tasks
        .get(task_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {task_id} not found")))?;
    Ok(super::response::from_task(&task))
}
