//! Required podcast_id validation for get_podcast.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn parse(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "podcast",
                "'podcast_id' is required for get_podcast",
                Some(vec!["podcast_id"]),
                Some(json!({"action": "get_podcast", "podcast_id": "abc12345"})),
            ));
        }
    }
}
