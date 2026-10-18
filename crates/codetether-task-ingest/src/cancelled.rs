//! Apply task cancelled notifications.
use codetether_a2a_worker_core::cloudevents::CloudEvent;
use codetether_task_queue::KnativeTaskQueue;
pub(crate) async fn apply(queue: &KnativeTaskQueue, event: &CloudEvent) {
    tracing::info!("Task cancellation event received");
    // Update task status if we have the task_id
    if let Some(task_id) = event.data.get("task_id").and_then(|v| v.as_str()) {
        let _ = queue.update_status(task_id, "cancelled").await;
        tracing::info!(task_id = %task_id, "Task cancelled");
    }
}
