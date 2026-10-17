//! Validate that the requested file already exists.
use super::ToolResult;
use serde_json::json;
use std::path::Path;
pub(super) fn check(path: &Path, file_path: &str) -> Option<ToolResult> {
    if !path.exists() {
        return Some(ToolResult::structured_error(
            "FILE_NOT_FOUND",
            "edit",
            &format!("File not found: {file_path}"),
            None,
            Some(json!({
                "hint": "Use an absolute path. List directory contents first to verify the file exists.",
                "filePath": file_path
            })),
        ));
    }

    None
}
