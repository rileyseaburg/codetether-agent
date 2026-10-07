//! Real in-process service regressions; no durable or live-runtime evidence.

use super::super::service::ingest;
use super::fixtures::{payload, setup};
use crate::{a2a::types::TaskState, bus::BusMessage};

#[tokio::test]
async fn accepted_progress_publishes_exact_task_identity() {
    let (tasks, bus, mut reader) = setup("processing").await;
    ingest(&tasks, &bus, "task-1", &payload()).await.unwrap();
    let envelope = reader.try_recv().expect("accepted output event");
    assert_eq!(envelope.topic, "task.task-1");
    assert_eq!(envelope.sender_id, "task-output");
    match envelope.message {
        BusMessage::TaskUpdate {
            task_id,
            state,
            message,
        } => {
            assert_eq!(task_id, "task-1");
            assert_eq!(state, TaskState::Working);
            assert_eq!(message.as_deref(), Some("progress"));
        }
        other => panic!("unexpected event: {other:?}"),
    }
    assert_eq!(tasks.get("task-1").await.unwrap().status, "working");
    assert!(reader.try_recv().is_none());
}
