//! Assertions for terminal release envelopes shared by lifecycle cases.

use crate::{
    a2a::types::TaskState,
    bus::{BusEnvelope, BusMessage},
};

pub(super) fn assert_terminal_event(event: BusEnvelope, success: bool) {
    assert_terminal_event_message(
        event,
        success,
        Some(if success {
            "partial work"
        } else {
            "Error: tool crashed"
        }),
    );
}

pub(super) fn assert_terminal_event_message(
    event: BusEnvelope,
    success: bool,
    expected_message: Option<&str>,
) {
    assert_eq!(event.topic, "task.task-1");
    assert_eq!(event.sender_id, "worker_task_release");
    match event.message {
        BusMessage::TaskUpdate {
            task_id,
            state,
            message,
        } => {
            assert_eq!(task_id, "task-1");
            assert_eq!(
                state,
                if success {
                    TaskState::Completed
                } else {
                    TaskState::Failed
                }
            );
            assert_eq!(message.as_deref(), expected_message);
            assert!(state.is_terminal());
        }
        payload => panic!("unexpected event: {payload:?}"),
    }
}
