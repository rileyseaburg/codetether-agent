//! Normalize the existing worker-release contract once for storage and events.

use super::ReleaseRequest;
use crate::a2a::types::TaskState;

pub(super) struct Outcome {
    pub status: &'static str,
    pub state: TaskState,
    pub message: Option<String>,
}

pub(super) fn from_request(req: &ReleaseRequest) -> Outcome {
    let completed = matches!(req.status.as_str(), "completed" | "success");
    let error = req.error.as_ref().map(|error| format!("Error: {error}"));
    Outcome {
        status: if completed { "completed" } else { "failed" },
        state: if completed {
            TaskState::Completed
        } else {
            TaskState::Failed
        },
        message: if completed {
            req.result.clone().or(error)
        } else {
            error.or(req.result.clone())
        },
    }
}
