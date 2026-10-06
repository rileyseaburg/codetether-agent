//! Queue status and terminal event state must have the same meaning.

use super::{super::outcome, request};
use crate::a2a::types::TaskState;

#[test]
fn release_status_and_event_agree() {
    for status in ["completed", "success", "failed", "error", "unknown"] {
        let result = outcome::from_request(&request(status));
        let success = matches!(status, "completed" | "success");
        assert_eq!(result.status, if success { "completed" } else { "failed" });
        let expected = if success {
            TaskState::Completed
        } else {
            TaskState::Failed
        };
        assert_eq!(result.state, expected);
        assert!(result.state.is_terminal());
    }
}
