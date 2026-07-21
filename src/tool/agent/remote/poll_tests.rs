//! Terminal-state classification tests for remote polling.

use super::finished;
use crate::a2a::types::TaskState;

#[test]
fn only_submitted_and_working_tasks_keep_polling() {
    assert!(!finished(TaskState::Submitted));
    assert!(!finished(TaskState::Working));
    for state in [
        TaskState::Completed,
        TaskState::Failed,
        TaskState::Cancelled,
        TaskState::InputRequired,
        TaskState::Rejected,
        TaskState::AuthRequired,
    ] {
        assert!(finished(state));
    }
}
