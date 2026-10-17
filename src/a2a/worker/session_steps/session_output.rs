//! Tool result and text output helpers for session steps.

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

pub(super) fn add_tool_result(
    session: &mut Session,
    tool_call_id: String,
    result: crate::tool::ToolResult,
) {
    let mut content = vec![ContentPart::ToolResult {
        tool_call_id,
        content: result.output,
    }];
    content.extend(crate::tool::result_images::content(Some(&result.metadata)));
    session.add_message(Message {
        role: Role::Tool,
        content,
    });
}
