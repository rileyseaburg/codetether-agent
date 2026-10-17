//! Require a nonempty edit-operation array.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(input: &Value) -> Result<&Vec<Value>, ToolResult> {
    match input.get("edits").and_then(|v| v.as_array()) {
        Some(arr) if !arr.is_empty() => Ok(arr),
        Some(_) => {
            return Err(ToolResult::structured_error(
                "INVALID_FIELD",
                "confirm_multiedit",
                "edits array must contain at least one edit operation",
                Some(vec!["edits"]),
                Some(
                    json!({"edits": [{"file": "path/to/file", "old_string": "old", "new_string": "new"}]}),
                ),
            ));
        }
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "confirm_multiedit",
                "edits is required and must be an array of edit objects with 'file', 'old_string', 'new_string' fields",
                Some(vec!["edits"]),
                Some(
                    json!({"edits": [{"file": "path/to/file", "old_string": "old", "new_string": "new"}]}),
                ),
            ));
        }
    }
}
