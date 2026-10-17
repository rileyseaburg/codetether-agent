//! Validate the required old_string argument.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(input: &Value, path: &str) -> Result<String, ToolResult> {
    match input.get("old_string").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "confirm_edit",
                "old_string is required (the exact string to replace)",
                Some(vec!["old_string"]),
                Some(
                    json!({"path": path, "old_string": "text to find", "new_string": "replacement"}),
                ),
            ));
        }
    }
}
