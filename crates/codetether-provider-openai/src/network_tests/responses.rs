//! Minimal valid SDK completion and SSE response payloads.

use super::*;
pub(super) fn chat() -> String {
    json!({"id":"chat", "object":"chat.completion", "created":1, "model":"fixture",
        "choices":[{"index":0,"message":{"role":"assistant","content":"ok"},"finish_reason":"stop"}],
        "usage":{"prompt_tokens":2,"completion_tokens":3,"total_tokens":5}}).to_string()
}
pub(super) fn frame(delta: Value) -> String {
    format!(
        "data: {}\n\n",
        json!({"id":"chat", "object":"chat.completion.chunk", "created":1,
        "model":"fixture", "choices":[{"index":0,"delta":delta,"finish_reason":null}]})
    )
}
