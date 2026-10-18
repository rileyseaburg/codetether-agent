//! Native session synchronization with shared transcript projection.
#[path = "message_text/sync.rs"]
mod sync;
pub use codetether_chat_transcript::extract_message_text;
pub use sync::sync_messages_from_session;
pub(crate) use sync::sync_messages_from_source;

#[cfg(test)]
include!("message_text_test_imports.rs");
