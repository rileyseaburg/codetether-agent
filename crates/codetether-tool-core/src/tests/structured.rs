//! Focused adapter/result contract tests.
use crate::ToolResult;
use serde_json::json;

#[test]
fn structured_errors_keep_machine_readable_fields() {
    let result = ToolResult::structured_error(
        "MISSING",
        "read",
        "path required",
        Some(vec!["path"]),
        Some(json!({"path":"file.txt"})),
    );
    assert!(!result.success);
    let value: serde_json::Value = serde_json::from_str(&result.output).unwrap();
    assert_eq!(
        value,
        json!({"error":{"code":"MISSING", "tool":"read", "message":"path required",
        "missing_fields":["path"], "example":{"path":"file.txt"}}})
    );
    assert_eq!(result.metadata["error_code"], "MISSING");
    assert_eq!(result.metadata["tool"], "read");
}
