//! Assistant message payload with optional text.
use crate::*;
pub(super) fn convert(msg: &Message) -> Value {
    let (text_parts, tool_calls) = super::assistant_parts::collect(msg);
    let content = text_parts.join("\n");
    let mut m = json!({"role": "assistant"});
    if !content.is_empty() || tool_calls.is_empty() {
        m["content"] = json!(content);
    }
    if !tool_calls.is_empty() {
        m["tool_calls"] = json!(tool_calls);
    }
    return m;
}
