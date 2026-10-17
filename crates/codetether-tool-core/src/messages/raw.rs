//! Tool-result messages preserving raw text and image attachments.
use super::*;
/// Construct a tool reply without adding feedback text around its output.
///
/// # Arguments
/// * `tool_call_id` - Provider tool-call identifier being answered.
/// * `result` - Raw tool output and image metadata.
/// # Returns
/// A Tool-role message carrying raw output followed by image content.
/// # Examples
/// ```rust
/// use codetether_tool_core::{ToolResult, messages::raw_tool_result};
/// let message = raw_tool_result("call".into(), ToolResult::success("done"));
/// assert!(matches!(message.role, codetether_provider_core::Role::Tool));
/// ```

pub fn raw_tool_result(tool_call_id: String, result: crate::ToolResult) -> Message {
    let mut content = vec![ContentPart::ToolResult {
        tool_call_id,
        content: result.output,
    }];
    content.extend(crate::images::content(Some(&result.metadata)));
    Message {
        role: Role::Tool,
        content,
    }
}
