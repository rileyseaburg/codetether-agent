//! Validate the required path argument.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(input: &Value) -> Result<String, ToolResult> {
    match input.get("path").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "confirm_edit",
                "path is required (path to the file to edit)",
                Some(vec!["path"]),
                Some(
                    json!({"path": "src/main.rs", "old_string": "old text", "new_string": "new text"}),
                ),
            ));
        }
    }
}
