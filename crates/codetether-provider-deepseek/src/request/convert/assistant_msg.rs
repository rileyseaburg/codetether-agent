//! assistant msg conversion.
use super::*;

pub(super) fn assistant_msg(m: &Message) -> Value {
    let txt = convert_helpers::collect_text(m);
    let calls = convert_helpers::collect_calls(m);
    let reason = convert_helpers::collect_thinking(m);
    tracing::debug!(tool_calls = calls.len(), "DeepSeek convert assistant msg");
    if calls.is_empty() {
        let mut val = json!({"role": "assistant", "content": txt});
        if !reason.is_empty() {
            val["reasoning_content"] = json!(reason);
        }
        val
    } else {
        json!({
            "role": "assistant",
            "content": if txt.is_empty() { Value::Null } else { json!(txt) },
            "reasoning_content": if reason.is_empty() { Value::Null } else { json!(reason) },
            "tool_calls": calls
        })
    }
}
