//! Required script validation for create_episode.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn parse(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "podcast",
                "'script' is required for create_episode — this is the text that will be converted to speech",
                Some(vec!["script"]),
                Some(
                    json!({"action": "create_episode", "podcast_id": "abc12345", "title": "Episode 1", "script": "Hello listeners..."}),
                ),
            ));
        }
    }
}
