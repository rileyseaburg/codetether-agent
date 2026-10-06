//! Resolve task notifications against the queue rather than forwarding bus data.

use crate::a2a::types::TaskState;
use crate::bus::{BusEnvelope, BusMessage};
use crate::server::{KnativeTask, KnativeTaskQueue};
use axum::response::sse::Event;

pub(super) async fn queued_task(
    queue: &KnativeTaskQueue,
    envelope: &BusEnvelope,
) -> Option<KnativeTask> {
    let BusMessage::TaskUpdate {
        task_id,
        state: TaskState::Submitted,
        ..
    } = &envelope.message
    else {
        return None;
    };
    if envelope.topic != format!("task.{task_id}") {
        return None;
    }
    queue
        .get(task_id)
        .await
        .filter(|task| task.status == "pending" || task.status == "queued")
}

pub(super) fn task_event(task: &KnativeTask) -> Option<Event> {
    match serde_json::to_string(task) {
        Ok(payload) => Some(Event::default().event("task").data(payload)),
        Err(error) => {
            tracing::error!(task_id = %task.task_id, %error, "Task payload serialization failed");
            None
        }
    }
}
