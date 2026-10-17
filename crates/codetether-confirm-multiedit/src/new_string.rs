//! Validate the new_string field of a requested edit.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(
    edit_val: &Value,
    i: usize,
    file: &str,
    old_string: &str,
) -> Result<String, ToolResult> {
    match edit_val.get("new_string").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "INVALID_FIELD",
                "confirm_multiedit",
                &format!("edits[{i}].new_string is required"),
                Some(vec!["new_string"]),
                Some(json!({"file": file, "old_string": old_string, "new_string": "replacement"})),
            ));
        }
    }
}
