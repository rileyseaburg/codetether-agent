//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_handles_raw_json() {
    let raw = r#"{"project": "test", "feature": "foo"}"#;
    assert!(extract_json(raw).is_some());
}
