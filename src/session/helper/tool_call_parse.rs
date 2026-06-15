//! Parse native tool calls out of a completion message.
//!
//! Separates well-formed tool calls (parsed arguments) from calls whose
//! arguments failed to parse — typically because the model's output was
//! truncated by a `max_tokens` cutoff mid-call.

use crate::provider::ContentPart;

/// Split a message's tool calls into `(parsed, truncated_ids)`.
///
/// `parsed` holds `(id, name, arguments)` tuples ready for execution.
/// `truncated_ids` holds `(id, name)` for calls whose JSON arguments could not
/// be parsed, so the caller can surface a corrective error to the model.
pub fn parse_tool_calls(
    content: &[ContentPart],
) -> (
    Vec<(String, String, serde_json::Value)>,
    Vec<(String, String)>,
) {
    let mut truncated_tool_ids: Vec<(String, String)> = Vec::new();
    let tool_calls = content
        .iter()
        .filter_map(|part| {
            let ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } = part
            else {
                return None;
            };
            match serde_json::from_str::<serde_json::Value>(arguments) {
                Ok(args) => Some((id.clone(), name.clone(), args)),
                Err(e) => {
                    tracing::warn!(
                        tool = %name,
                        tool_call_id = %id,
                        args_len = arguments.len(),
                        error = %e,
                        "Tool call arguments failed to parse (likely truncated by max_tokens)"
                    );
                    truncated_tool_ids.push((id.clone(), name.clone()));
                    None
                }
            }
        })
        .collect();
    (tool_calls, truncated_tool_ids)
}
