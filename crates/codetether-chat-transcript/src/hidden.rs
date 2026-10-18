//! Hide internal context-compression markers in the visible transcript.
use codetether_provider_core::{ContentPart, Message, Role};

pub(crate) fn is_hidden_context_marker(message: &Message) -> bool {
    matches!(
        (&message.role, message.content.as_slice()),
        (
            Role::Assistant,
            [ContentPart::Text { text }]
        ) if text.starts_with("[AUTO CONTEXT COMPRESSION]")
            || text.starts_with("[CONTEXT TRUNCATED]")
    )
}
