//! Validate the file path field of an edit operation.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(entry: &Value, i: usize) -> Result<String, ToolResult> {
    match entry.get("file").and_then(|v| v.as_str()) {
        Some(f) => Ok(f.to_string()),
        None => {
            return Err(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "multiedit",
                &format!("edits[{i}]: missing or non-string 'file' field"),
                Some(vec!["edits", "file"]),
                Some(json!({"file": "src/example.rs", "old_string": "...", "new_string": "..."})),
            ));
        }
    }
}
