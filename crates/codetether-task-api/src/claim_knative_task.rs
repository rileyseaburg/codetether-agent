//! claim knative task queue API operation.
use axum::{Json, http::StatusCode};
use codetether_task_queue::{KnativeTask, KnativeTaskQueue};

/// Claim/start processing a Knative task
pub async fn claim_knative_task(
    queue: &KnativeTaskQueue,
    task_id: String,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    // Update status to processing
    let updated = queue.update_status(&task_id, "processing").await;
    if !updated {
        return Err((StatusCode::NOT_FOUND, format!("Task {} not found", task_id)));
    }

    queue
        .get(&task_id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))
}
