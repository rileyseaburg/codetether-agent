//! Session recording for executed tool calls.

use super::image_inject::inject_tool_image;
use super::messages::PendingToolCall;
use crate::agent::ToolUse;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::tool::ToolResult;

/// Append executed tool results to the session history.
pub(super) fn record_results(
    session: &mut Session,
    tool_calls: Vec<PendingToolCall>,
    results: Vec<ToolResult>,
) {
    for ((id, name, arguments), result) in tool_calls.into_iter().zip(results) {
        inject_tool_image(session, &result);
        session.tool_uses.push(ToolUse {
            id: id.clone(),
            name: name.clone(),
            input: arguments.clone(),
            output: result.output.clone(),
            success: result.success,
        });
        session.add_message(Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id,
                content: result.output,
            }],
        });
    }
}
