//! Tool result and text output helpers for session steps.

use std::sync::Arc;

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

pub(super) fn add_tool_result(session: &mut Session, tool_call_id: String, content: String) {
    session.add_message(Message { role: Role::Tool,
        content: vec![ContentPart::ToolResult { tool_call_id, content }] });
}

pub(super) fn append_text_output(
    parts: &[ContentPart],
    out: &mut String,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    for part in parts {
        if let ContentPart::Text { text } = part && !text.is_empty() {
            out.push_str(text);
            out.push('\n');
            if let Some(cb) = cb { cb(text.clone()); }
        }
    }
}
