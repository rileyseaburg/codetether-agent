//! Required episode_id validation for delete_episode.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn parse(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "podcast",
                "'episode_id' is required for delete_episode",
                Some(vec!["episode_id"]),
                Some(
                    json!({"action": "delete_episode", "podcast_id": "abc12345", "episode_id": "xyz789"}),
                ),
            ));
        }
    }
}
