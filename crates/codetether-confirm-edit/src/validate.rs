//! Reject missing or ambiguous replacement targets.
use super::{ToolResult, input::Input};
use serde_json::json;
pub(super) fn validate(input: &Input, content: &str) -> Option<ToolResult> {
    let count = content.matches(input.old_string.as_str()).count();

    if count == 0 {
        return Some(ToolResult::structured_error(
            "NOT_FOUND",
            "confirm_edit",
            "old_string not found in file. Make sure it matches exactly, including whitespace.",
            None,
            Some(json!({
                "hint": "Use the 'read' tool first to see the exact content",
                "path": input.path,
                "old_string": "<copy exact text from file>",
                "new_string": input.new_string
            })),
        ));
    }

    if count > 1 {
        return Some(ToolResult::structured_error(
            "AMBIGUOUS_MATCH",
            "confirm_edit",
            &format!(
                "old_string found {} times. Include more context to uniquely identify the location.",
                count
            ),
            None,
            Some(json!({
                "hint": "Include 3+ lines of context before and after the target text",
                "matches_found": count
            })),
        ));
    }

    None
}
