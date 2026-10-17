//! Validate the file field of a requested edit.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(edit_val: &Value, i: usize) -> Result<String, ToolResult> {
    match edit_val.get("file").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "INVALID_FIELD",
                "confirm_multiedit",
                &format!("edits[{i}].file is required"),
                Some(vec!["file"]),
                Some(json!({"file": "path/to/file", "old_string": "old", "new_string": "new"})),
            ));
        }
    }
}
