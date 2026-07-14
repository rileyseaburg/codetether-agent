//! Provider tool-call extraction for worker session steps.

use crate::provider::ContentPart;

pub(in crate::a2a::worker::session_steps) type ToolCall = (String, String, serde_json::Value);

pub(in crate::a2a::worker::session_steps) fn collect_tool_calls(
    parts: &[ContentPart],
) -> Vec<ToolCall> {
    parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some((
                id.clone(),
                name.clone(),
                serde_json::from_str(arguments).unwrap_or(serde_json::json!({})),
            )),
            _ => None,
        })
        .collect()
}
