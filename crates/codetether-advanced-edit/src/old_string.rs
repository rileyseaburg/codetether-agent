//! Validate the old_string input and preserve its structured error.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(params: &Value, file_path: &str) -> Result<String, ToolResult> {
    match params.get("oldString").and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "edit",
                "oldString is required (the exact text to find and replace)",
                Some(vec!["oldString"]),
                Some(json!({
                    "filePath": file_path,
                    "oldString": "text to find in file",
                    "newString": "replacement text"
                })),
            ));
        }
    }
}
