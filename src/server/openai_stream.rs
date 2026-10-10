//! OpenAI-compatible stream response helpers.

/// Selects the terminal reason for an OpenAI-compatible streamed response.
pub(super) fn finish_reason(saw_tool_calls: bool, saw_text: bool) -> &'static str {
    if saw_tool_calls && !saw_text {
        "tool_calls"
    } else {
        "stop"
    }
}

/// Delta carrying model reasoning, using the `reasoning_content` field
/// that OpenAI-compatible clients read for thinking output.
pub(super) fn reasoning_delta(text: String) -> serde_json::Value {
    serde_json::json!({ "reasoning_content": text })
}
