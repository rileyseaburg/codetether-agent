//! The synthetic `[CONTEXT TRUNCATED]` marker message.

use crate::provider::{ContentPart, Message, Role};

/// Build the assistant marker prepended by terminal truncation.
pub(super) fn truncation_marker(dropped_prefix: bool) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: if dropped_prefix {
                "[CONTEXT TRUNCATED]\nOlder conversation was dropped to keep the request \
                 under the model's context window. Some retained tool output may also be \
                 shortened with head/tail snippets."
                    .to_string()
            } else {
                "[CONTEXT TRUNCATED]\nRecent tool output was too large for the model's \
                 context window and was shortened with head/tail snippets."
                    .to_string()
            },
        }],
    }
}
