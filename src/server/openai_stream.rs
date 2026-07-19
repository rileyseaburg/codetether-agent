//! OpenAI-compatible stream response helpers.

/// Selects the terminal reason for an OpenAI-compatible streamed response.
pub(super) fn finish_reason(saw_tool_calls: bool, saw_text: bool) -> &'static str {
    if saw_tool_calls && !saw_text {
        "tool_calls"
    } else {
        "stop"
    }
}
