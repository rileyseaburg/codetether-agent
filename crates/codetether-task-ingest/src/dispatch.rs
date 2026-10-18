//! Dispatch recognized event types without changing unknown-type acknowledgement.
use codetether_a2a_worker_core::cloudevents::CloudEvent;
use codetether_task_queue::KnativeTaskQueue;
pub(crate) async fn apply(queue: &KnativeTaskQueue, event: &CloudEvent) {
    match event.event_type.as_str() {
        "codetether.task.created" | "task.created" => crate::created::apply(queue, event).await,
        "codetether.task.updated" | "task.updated" => crate::updated::apply(queue, event).await,
        "codetether.task.cancelled" => crate::cancelled::apply(queue, event).await,
        _ => {
            tracing::warn!(event_type = %event.event_type, "Unknown CloudEvent type");
        }
    }
}
