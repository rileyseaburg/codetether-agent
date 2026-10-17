//! Append the shared raw tool reply to the application session.
use crate::session::Session;
pub(super) fn add_tool_result(
    session: &mut Session,
    tool_call_id: String,
    result: crate::tool::ToolResult,
) {
    session.add_message(codetether_tool_core::messages::raw_tool_result(
        tool_call_id,
        result,
    ));
}
