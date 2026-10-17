//! Assistant message encoding including empty reasoning content.
use crate::*;
pub(super) fn convert(msg: &Message) -> Value {
    let text = super::content::text(msg);
    let tool_calls = super::content::calls(msg);
    if tool_calls.is_empty() {
        json!({"role": "assistant", "content": text})
    } else {
        // Moonshot requires reasoning_content for K2.5 thinking models
        // Include empty string when we don't have the original
        json!({
            "role": "assistant",
            "content": if text.is_empty() { "".to_string() } else { text },
            "reasoning_content": "",
            "tool_calls": tool_calls
        })
    }
}
