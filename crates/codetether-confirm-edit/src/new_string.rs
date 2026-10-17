//! Validate the required new_string argument.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(input: &Value, path: &str, old_string: &str) -> Result<String, ToolResult> {
    match input.get("new_string").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "confirm_edit",
                "new_string is required (the replacement text)",
                Some(vec!["new_string"]),
                Some(json!({"path": path, "old_string": old_string, "new_string": "replacement"})),
            ));
        }
    }
}
