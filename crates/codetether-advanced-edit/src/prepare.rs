//! Map matching failures to the original structured editing error.
use super::{ToolResult, replace::replace};
use serde_json::json;
pub(super) fn prepare(
    content: &str,
    old_string: &str,
    new_string: &str,
    replace_all: bool,
    file_path: &str,
) -> Result<String, ToolResult> {
    match replace(&content, &old_string, &new_string, replace_all) {
        Ok(c) => Ok(c),
        Err(_) => {
            return Err(ToolResult::structured_error(
                "NOT_FOUND",
                "edit",
                "oldString not found in file content. Provide more surrounding context or check for typos, whitespace, and indentation.",
                None,
                Some(json!({
                    "hint": "Read the file first to see its exact content, then copy the text you want to replace verbatim including whitespace.",
                    "filePath": file_path,
                    "oldString": "<copy exact text from file including whitespace and indentation>",
                    "newString": "replacement text"
                })),
            ));
        }
    }
}
