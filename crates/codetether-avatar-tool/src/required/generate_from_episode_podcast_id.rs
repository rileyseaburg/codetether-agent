//! Required podcast_id validation for generate_from_episode.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn parse(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "avatar",
                "'podcast_id' is required for generate_from_episode",
                Some(vec!["podcast_id"]),
                Some(
                    json!({"action": "generate_from_episode", "podcast_id": "abc123", "episode_id": "xyz789"}),
                ),
            ));
        }
    }
}
