//! local task state to proto conversion.
use super::*;

pub(super) fn local_task_state_to_proto(state: local::TaskState) -> proto::TaskState {
    match state {
        local::TaskState::Submitted => proto::TaskState::Submitted,
        local::TaskState::Working => proto::TaskState::Working,
        local::TaskState::Completed => proto::TaskState::Completed,
        local::TaskState::Failed => proto::TaskState::Failed,
        local::TaskState::Cancelled => proto::TaskState::Cancelled,
        local::TaskState::InputRequired => proto::TaskState::InputRequired,
        local::TaskState::Rejected => proto::TaskState::Rejected,
        local::TaskState::AuthRequired => proto::TaskState::AuthRequired,
    }
}
