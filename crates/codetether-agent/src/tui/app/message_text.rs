//! Native session synchronization with shared transcript projection.
#[path = "message_text/sync.rs"]
mod sync;
pub use codetether_chat_transcript::extract_message_text;
pub(crate) use codetether_chat_transcript::provider_messages_to_chat_messages;
pub use sync::sync_messages_from_session;
pub(crate) use sync::sync_messages_from_source;
