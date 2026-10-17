//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_handles_line_comments() {
    let text = "{\n  \"project\": \"test\", // this is the project\n  \"feature\": \"foo\"\n}";
    let result = extract_json(text).unwrap();
    serde_json::from_str::<serde_json::Value>(&result).unwrap();
}
