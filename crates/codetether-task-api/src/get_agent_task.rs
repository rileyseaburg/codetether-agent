//! get agent task queue API operation.
use axum::{Json, http::StatusCode};
use codetether_task_queue::{KnativeTask, KnativeTaskQueue};

/// Get a single agent task by ID
pub async fn get_agent_task(
    queue: &KnativeTaskQueue,
    task_id: String,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    queue
        .get(&task_id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))
}
