//! has open tool call.
use super::*;

pub(super) fn has_open_tool_call(msg: &Message) -> bool {
    msg.role == Role::Assistant
        && msg
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { .. }))
}
