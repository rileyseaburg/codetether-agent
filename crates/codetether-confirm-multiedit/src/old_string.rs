//! Validate the old_string field of a requested edit.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(edit_val: &Value, i: usize, file: &str) -> Result<String, ToolResult> {
    match edit_val.get("old_string").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "INVALID_FIELD",
                "confirm_multiedit",
                &format!("edits[{i}].old_string is required"),
                Some(vec!["old_string"]),
                Some(
                    json!({"file": file, "old_string": "text to find", "new_string": "replacement"}),
                ),
            ));
        }
    }
}
