//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_handles_prose_wrapper() {
    let text = "Sure! Here is the PRD:\n\n{\"project\": \"x\", \"feature\": \"y\"}\n\nLet me know if you need changes.";
    let result = extract_json(text).unwrap();
    assert!(result.contains("\"project\""));
}
