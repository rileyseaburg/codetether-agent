//! Tool result and text output helpers for session steps.

use std::sync::Arc;

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

pub(super) fn append_text_output(
    parts: &[ContentPart],
    out: &mut String,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    for part in parts {
        if let ContentPart::Text { text } = part
            && !text.is_empty()
        {
            out.push_str(text);
            out.push('\n');
            if let Some(cb) = cb {
                cb(text.clone());
            }
        }
    }
}
