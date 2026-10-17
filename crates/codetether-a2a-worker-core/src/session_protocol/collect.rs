//! Provider tool-call extraction for worker session steps.

use codetether_provider_core::ContentPart;

/// Tool-call identifier, tool name and decoded arguments.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::session_protocol::ToolCall;
/// let call: ToolCall = ("id".into(), "read".into(), serde_json::json!({}));
/// assert_eq!(call.1, "read");
/// ```
pub type ToolCall = (String, String, serde_json::Value);

/// Extract tool calls, substituting an empty object for invalid argument JSON.
///
/// # Arguments
/// * `parts` - Provider response content in its original order.
/// # Returns
/// Tool identifiers, names and decoded arguments in response order.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::session_protocol::collect_tool_calls;
/// assert!(collect_tool_calls(&[]).is_empty());
/// ```
pub fn collect_tool_calls(parts: &[ContentPart]) -> Vec<ToolCall> {
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
