//! Terminal-state classification for polled A2A tasks.

use crate::a2a::types::TaskState;

/// Terminal task states stop the poll loop.
pub(super) fn finished(state: TaskState) -> bool {
    match state {
        TaskState::Submitted | TaskState::Working => false,
        TaskState::Completed
        | TaskState::Failed
        | TaskState::Cancelled
        | TaskState::InputRequired
        | TaskState::Rejected
        | TaskState::AuthRequired => true,
    }
}
