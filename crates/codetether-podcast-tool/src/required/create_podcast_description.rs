//! Required description validation for create_podcast.
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn parse(value: Option<String>) -> Result<String, ToolResult> {
    match value {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_PARAM",
                "podcast",
                "'description' is required for create_podcast",
                Some(vec!["description"]),
                Some(
                    json!({"action": "create_podcast", "title": "My Podcast", "description": "A tech podcast"}),
                ),
            ));
        }
    }
}
