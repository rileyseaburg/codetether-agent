//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_handles_trailing_commas() {
    let text = r#"{"project": "test", "feature": "foo",}"#;
    let result = extract_json(text).unwrap();
    assert!(result.contains("test"));
    // Verify it actually parses
    serde_json::from_str::<serde_json::Value>(&result).unwrap();
}
