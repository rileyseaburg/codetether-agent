//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_handles_markdown_wrapped() {
    let wrapped = "Here is the PRD:\n```json\n{\"project\": \"test\"}\n```\nDone.";
    let result = extract_json(wrapped).unwrap();
    assert!(result.contains("test"));
}
