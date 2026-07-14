//! Shared rendering for compressed history messages.

use crate::provider::{ContentPart, Message, Role};

pub(super) fn install(messages: &mut Vec<Message>, tail: Vec<Message>, summary: String, toc: &str) {
    let summary_msg = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: format!(
                "[AUTO CONTEXT COMPRESSION]\nOlder conversation + tool output was compressed \
                 to fit the model context window.\n\n{summary}{toc}\n\n\
                 [RECOVERY] If you need specific details that this summary \
                 dropped (exact file paths, prior tool output, earlier user \
                 instructions, numeric values), use a targeted `session_recall` \
                 query only when the user allows history access and the detail \
                 is absent from user-designated repository sources."
            ),
        }],
    };
    let mut new_messages = Vec::with_capacity(1 + tail.len());
    new_messages.push(summary_msg);
    new_messages.extend(tail);
    *messages = new_messages;
}
