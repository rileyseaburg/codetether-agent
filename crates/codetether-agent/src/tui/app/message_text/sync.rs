//! Native session binding for shared transcript synchronization.
#[path = "sync/view.rs"]
mod view;
pub use codetether_chat_transcript::sync::session::sync_messages_from_session;
pub(crate) use codetether_chat_transcript::sync::session::sync_messages_from_source;
