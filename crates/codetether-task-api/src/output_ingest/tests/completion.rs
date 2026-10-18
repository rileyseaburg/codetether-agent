//! Terminal receipt fixture, containing no credentials.

use crate::release::ReleaseRequest;

pub(super) fn request(status: &str) -> ReleaseRequest {
    ReleaseRequest {
        task_id: "task-1".into(),
        status: status.into(),
        result: Some("final result".into()),
        error: None,
        session_id: Some("original-session".into()),
        diagnostics: Some(serde_json::json!({"phase": "done"})),
    }
}
