//! Construct a call-associated tool result message.
use super::*;

/// Build a text-only tool message with feedback status.
///
/// # Arguments
/// * `tool_call_id` - Identifier of the tool call being answered.
/// * `tool` - Tool name included in feedback.
/// * `success` - Whether execution succeeded.
/// * `output` - Complete tool output, without additional truncation.
/// # Returns
/// A Role::Tool message whose first part carries the original call ID.
/// # Examples
/// ```
/// let message = codetether_tool_core::messages::tool_result_with_status("id".into(), "bash", true, "done".into());
/// assert!(matches!(message.role, codetether_provider_core::Role::Tool));
/// ```
pub fn tool_result_with_status(
    tool_call_id: String,
    tool: &str,
    success: bool,
    output: String,
) -> Message {
    tool_result_with_metadata(tool_call_id, tool, success, output, None)
}
