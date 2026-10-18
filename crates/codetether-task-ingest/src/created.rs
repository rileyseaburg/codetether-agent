//! Queue a created task using the event fields and timestamp.
use codetether_a2a_worker_core::cloudevents::CloudEvent;
use codetether_task_queue::{KnativeTask, KnativeTaskQueue};
pub(crate) async fn apply(queue: &KnativeTaskQueue, event: &CloudEvent) {
    let (task_id, title, description, agent_type, priority) = crate::fields::extract(event);
    let task = KnativeTask {
        task_id: task_id.clone(),
        title,
        description,
        agent_type,
        model: event
            .data
            .get("model")
            .and_then(|v| v.as_str())
            .map(str::to_owned),
        metadata: event.data.get("metadata").cloned(),
        priority,
        received_at: chrono::Utc::now(),
        status: "queued".to_string(),
        completion: Default::default(),
    };

    queue.push(task).await;
    tracing::info!(task_id = %task_id, "Task queued for execution");
}
