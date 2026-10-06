//! Distinct completion receipts for concurrent release tests.

use super::{super::ReleaseRequest, request};

pub(super) fn receipt(index: usize) -> ReleaseRequest {
    let mut req = request(if index % 2 == 0 {
        "completed"
    } else {
        "failed"
    });
    req.result = Some(format!("result-{index}"));
    req.error = Some(format!("error-{index}"));
    req.session_id = Some(format!("session-{index}"));
    req.diagnostics = Some(serde_json::json!({"release": index}));
    req
}
