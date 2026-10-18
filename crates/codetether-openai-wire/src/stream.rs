//! OpenAI-compatible chunk and delta payload construction.

/// Construct one OpenAI chat-completion stream chunk.
/// # Arguments
/// * `id` - Completion identifier.
/// * `created` - Creation time in Unix seconds.
/// * `model` - Response model identifier.
/// * `delta` - Incremental response content.
/// * `finish_reason` - Terminal reason or None for an intermediate chunk.
/// # Returns
/// JSON containing one choice with the supplied delta and finish reason.
/// # Examples
/// ```
/// let chunk = codetether_openai_wire::openai_stream_chunk("id", 1, "model", serde_json::json!({}), None);
/// assert_eq!(chunk["object"], "chat.completion.chunk");
/// ```
pub fn openai_stream_chunk(
    id: &str,
    created: i64,
    model: &str,
    delta: serde_json::Value,
    finish_reason: Option<&str>,
) -> serde_json::Value {
    let finish_reason = finish_reason
        .map(|value| serde_json::Value::String(value.to_string()))
        .unwrap_or(serde_json::Value::Null);

    serde_json::json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": delta,
            "finish_reason": finish_reason,
        }]
    })
}

/// Selects the terminal reason for an OpenAI-compatible streamed response.
/// # Arguments
/// * `saw_tool_calls` - Whether the response included tool calls.
/// * `saw_text` - Whether it also included text.
/// # Returns
/// tool_calls for tool-only output, otherwise stop.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_wire::stream::finish_reason(true, false), "tool_calls");
/// ```
pub fn finish_reason(saw_tool_calls: bool, saw_text: bool) -> &'static str {
    if saw_tool_calls && !saw_text {
        "tool_calls"
    } else {
        "stop"
    }
}

/// Delta carrying model reasoning, using the `reasoning_content` field
/// that OpenAI-compatible clients read for thinking output.
/// # Arguments
/// * `text` - Reasoning content emitted by the model.
/// # Returns
/// A delta with the reasoning_content field.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_wire::stream::reasoning_delta("thinking".into())["reasoning_content"], "thinking");
/// ```
pub fn reasoning_delta(text: String) -> serde_json::Value {
    serde_json::json!({ "reasoning_content": text })
}
