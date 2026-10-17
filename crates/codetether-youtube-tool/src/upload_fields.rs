//! Required fields for the upload action.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(super) fn file_path(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "youtube",
                "'file_path' is required for upload_video",
                Some(vec!["file_path"]),
                Some(
                    json!({"action": "upload_video", "file_path": "/path/to/video.mp4", "title": "My Video"}),
                ),
            ));
        }
    }
}
pub(super) fn title(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "youtube",
                "'title' is required for upload_video",
                Some(vec!["title"]),
                Some(
                    json!({"action": "upload_video", "file_path": "/path/to/video.mp4", "title": "My Video"}),
                ),
            ));
        }
    }
}
