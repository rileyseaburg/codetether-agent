use serde_json::json;

use crate::tool::ToolResult;

use super::candidates;

pub fn not_found(content: &str, old: &str) -> ToolResult {
    let closest = candidates::nearest(content, old).unwrap_or_default();
    ToolResult::structured_error(
        "NOT_FOUND",
        "edit",
        "old_string not found. Tried exact and whitespace/indentation-tolerant matching; closest candidate is included.",
        None,
        Some(json!({
            "hint": "Use the closest_candidate as old_string, or add surrounding context",
            "closest_candidate": closest,
            "old_string": "<copy exact text from file including whitespace>"
        })),
    )
}
