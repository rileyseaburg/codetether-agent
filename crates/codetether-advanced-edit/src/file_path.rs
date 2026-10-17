//! Validate the file_path input and preserve its structured error.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(params: &Value) -> Result<String, ToolResult> {
    let example = json!({
        "filePath": "/absolute/path/to/file.rs",
        "oldString": "text to find",
        "newString": "replacement text"
    });

    match params.get("filePath").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => Ok(s.to_string()),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "edit",
                "filePath is required and must be a non-empty string (absolute path to the file)",
                Some(vec!["filePath"]),
                Some(example),
            ));
        }
    }
}
