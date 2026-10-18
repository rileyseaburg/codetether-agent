//! Parse native calls while retaining malformed IDs for corrective feedback.
use crate::parse_arguments;
use codetether_provider_core::ContentPart;

/// Split a message's tool calls into `(parsed, truncated_ids)`.
///
/// `parsed` holds `(id, name, arguments)` tuples ready for execution.
/// `truncated_ids` holds `(id, name)` for calls whose JSON arguments could not
/// be parsed, so the caller can surface a corrective error to the model.
/// # Arguments
/// Receives ordered provider content parts.
/// # Returns
/// Parsed (ID, name, JSON) calls and malformed (ID, name) pairs.
/// # Examples
/// ```
/// assert!(codetether_tool_call_parser::parse_tool_calls(&[]).0.is_empty());
/// ```
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
            match parse_arguments(arguments) {
                Ok(args) => Some((id.clone(), name.clone(), args)),
                Err(e) => {
                    tracing::warn!(target: "codetether_agent::session::helper::tool_modules::tool_call_parse",
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
