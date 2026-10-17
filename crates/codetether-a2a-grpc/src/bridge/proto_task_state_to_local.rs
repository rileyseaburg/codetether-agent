//! proto task state to local conversion.
use super::*;

pub(super) fn proto_task_state_to_local(state: proto::TaskState) -> local::TaskState {
    match state {
        proto::TaskState::Submitted => local::TaskState::Submitted,
        proto::TaskState::Working => local::TaskState::Working,
        proto::TaskState::Completed => local::TaskState::Completed,
        proto::TaskState::Failed => local::TaskState::Failed,
        proto::TaskState::Cancelled => local::TaskState::Cancelled,
        proto::TaskState::InputRequired => local::TaskState::InputRequired,
        proto::TaskState::Rejected => local::TaskState::Rejected,
        proto::TaskState::AuthRequired => local::TaskState::AuthRequired,
        proto::TaskState::Unspecified => local::TaskState::Submitted,
    }
}
