//! Lookup and presentation of the process-local completion receipt.

use axum::http::StatusCode;
use codetether_task_queue::KnativeTaskQueue;

/// Read a task receipt without mutating its lifecycle.
///
/// # Errors
/// Returns HTTP 404 when the queue has no task with the requested identity.
/// # Arguments
/// * `tasks` - Process-local queue.
/// * `task_id` - Task whose accepted receipt is requested.
/// # Returns
/// Compatible task output JSON with available receipt fields.
/// # Examples
/// ```no_run
/// # async fn example(q: &codetether_task_queue::KnativeTaskQueue) {
/// let _ = codetether_task_api::output_poll::snapshot(q, "task-1").await;
/// # }
/// ```
pub async fn snapshot(
    tasks: &KnativeTaskQueue,
    task_id: &str,
) -> Result<serde_json::Value, (StatusCode, String)> {
    let task = tasks
        .get(task_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {task_id} not found")))?;
    Ok(super::response::from_task(&task))
}
