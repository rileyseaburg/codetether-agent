//! Validate the new_string input and preserve its structured error.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(
    params: &Value,
    file_path: &str,
    old_string: &str,
) -> Result<String, ToolResult> {
    match params.get("newString").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "edit",
                "newString is required (the text to replace oldString with)",
                Some(vec!["newString"]),
                Some(json!({
                    "filePath": file_path,
                    "oldString": old_string,
                    "newString": "replacement text"
                })),
            ));
        }
    }
}
