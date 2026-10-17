//! Assistant request shape.
use crate::*;
pub(super) fn convert(msg: &Message) -> Value {
    let text = super::content::text(msg);
    let tool_calls = super::content::calls(msg);
    if tool_calls.is_empty() {
        json!({"role": "assistant", "content": text})
    } else {
        // For assistant with tool calls, content should be empty string or the text
        json!({
            "role": "assistant",
            "content": if text.is_empty() { "".to_string() } else { text },
            "tool_calls": tool_calls
        })
    }
}
