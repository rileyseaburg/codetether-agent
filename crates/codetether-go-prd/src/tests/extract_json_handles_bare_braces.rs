//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_handles_bare_braces() {
    let text = "The result is: {\"project\": \"test\"} and that's it.";
    let result = extract_json(text).unwrap();
    assert!(result.contains("test"));
}
