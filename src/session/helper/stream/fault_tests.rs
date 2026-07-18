//! Unit tests for transient stream-fault classification.

use super::is_transient;

#[test]
fn permanent_markers_override_transient_markers() {
    assert!(is_transient("connection reset by peer"));
    assert!(is_transient("HTTP 503 service unavailable"));
    assert!(is_transient("processing your request; you can retry"));
    assert!(!is_transient("context length exceeded"));
    assert!(!is_transient("401 unauthorized"));
    assert!(!is_transient("403 forbidden after 500ms"));
}
