//! Apply task updated notifications.
use codetether_a2a_worker_core::cloudevents::CloudEvent;
use codetether_task_queue::KnativeTaskQueue;
pub(crate) async fn apply(queue: &KnativeTaskQueue, event: &CloudEvent) {
    if let Some(task_id) = event.data.get("task_id").and_then(|v| v.as_str()) {
        let status = event
            .data
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("updated");
        let _ = queue.update_status(task_id, status).await;
        tracing::info!(task_id = %task_id, status = %status, "Task status updated");
    }
}
