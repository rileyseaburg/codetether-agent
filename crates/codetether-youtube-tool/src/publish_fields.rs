//! Required fields for the publish action.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(super) fn podcast_id(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "youtube",
                "'podcast_id' is required for publish_episode",
                Some(vec!["podcast_id"]),
                Some(
                    json!({"action": "publish_episode", "podcast_id": "abc123", "episode_id": "xyz789"}),
                ),
            ));
        }
    }
}
pub(super) fn episode_id(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "youtube",
                "'episode_id' is required for publish_episode",
                Some(vec!["episode_id"]),
                Some(
                    json!({"action": "publish_episode", "podcast_id": "abc123", "episode_id": "xyz789"}),
                ),
            ));
        }
    }
}
