//! list knative tasks queue API operation.
use axum::Json;
use codetether_task_queue::{KnativeTask, KnativeTaskQueue};

/// List all Knative tasks in queue
pub async fn list_knative_tasks(queue: &KnativeTaskQueue) -> Json<Vec<KnativeTask>> {
    Json(queue.list().await)
}
