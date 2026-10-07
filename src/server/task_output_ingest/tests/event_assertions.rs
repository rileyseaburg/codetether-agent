//! Exact task identity and ordered service-event assertions.

use crate::{
    a2a::types::TaskState,
    bus::{BusHandle, BusMessage},
};

pub(super) fn ordered(reader: &mut BusHandle, status: &str, accepted: bool) {
    if accepted {
        event(reader, "task-output", TaskState::Working);
    }
    let terminal = if status == "completed" {
        TaskState::Completed
    } else {
        TaskState::Failed
    };
    event(reader, "worker_task_release", terminal);
    assert!(reader.try_recv().is_none(), "no output after completion");
}

fn event(reader: &mut BusHandle, sender: &str, expected: TaskState) {
    let envelope = reader.try_recv().expect("expected service event");
    assert_eq!(envelope.topic, "task.task-1");
    assert_eq!(envelope.sender_id, sender);
    match envelope.message {
        BusMessage::TaskUpdate { task_id, state, .. } => {
            assert_eq!(task_id, "task-1");
            assert_eq!(state, expected);
        }
        other => panic!("unexpected event: {other:?}"),
    }
}
