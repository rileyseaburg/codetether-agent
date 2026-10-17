//! Required file_path validation for upload_model.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn parse(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "avatar",
                "'file_path' is required for upload_model",
                Some(vec!["file_path"]),
                Some(
                    json!({"action": "upload_model", "name": "Riley", "file_path": "/path/to/video.mp4"}),
                ),
            ));
        }
    }
}
