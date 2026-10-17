//! Construct a call-associated tool result message.
use super::*;

/// Build a tool message with feedback text and separate image attachments.
///
/// # Arguments
/// * `tool_call_id` - Identifier of the tool call being answered.
/// * `tool` - Tool name included in feedback.
/// * `success` - Whether execution succeeded.
/// * `output` - Complete tool output, without additional truncation.
/// * `metadata` - Optional canonical image metadata.
/// # Returns
/// A Role::Tool message whose first part carries the original call ID.
/// # Examples
/// ```
/// let message = codetether_tool_core::messages::tool_result_with_metadata("id".into(), "image", true, "done".into(), None);
/// assert!(matches!(message.role, codetether_provider_core::Role::Tool));
/// ```
pub fn tool_result_with_metadata(
    tool_call_id: String,
    tool: &str,
    success: bool,
    output: String,
    metadata: Option<&HashMap<String, Value>>,
) -> Message {
    let mut content = vec![ContentPart::ToolResult {
        tool_call_id,
        content: crate::feedback::render(tool, success, &output),
    }];
    content.extend(crate::images::content(metadata));
    Message {
        role: Role::Tool,
        content,
    }
}
