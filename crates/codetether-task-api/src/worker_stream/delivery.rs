//! Resolve task notifications against the authoritative process-local queue.

use axum::response::sse::Event;
use codetether_a2a_core::TaskState;
use codetether_bus_core::{BusEnvelope, BusMessage};
use codetether_task_queue::{KnativeTask, KnativeTaskQueue};

pub(super) async fn live(queue: &KnativeTaskQueue, envelope: BusEnvelope) -> Option<Event> {
    let BusMessage::TaskUpdate {
        task_id,
        state: TaskState::Submitted,
        ..
    } = envelope.message
    else {
        return None;
    };
    if envelope.topic != format!("task.{task_id}") {
        return None;
    }
    queued(queue, &task_id).await
}

pub(super) async fn queued(queue: &KnativeTaskQueue, task_id: &str) -> Option<Event> {
    let task = queue.get(task_id).await?;
    if !matches!(task.status.as_str(), "pending" | "queued") {
        return None;
    }
    event(task)
}

pub(super) fn event(task: KnativeTask) -> Option<Event> {
    match Event::default().event("task").json_data(&task) {
        Ok(event) => Some(event),
        Err(error) => {
            tracing::error!(task_id = %task.task_id, %error, "Cannot serialize worker task");
            None
        }
    }
}
