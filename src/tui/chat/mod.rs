//! Chat messages, retained payloads, and clean clipboard selection.

pub(crate) use codetether_chat_model::copy;
pub mod mermaid;
pub use codetether_chat_model as message;
pub use codetether_chat_model::payload;
pub use codetether_chat_sync as sync;
pub use codetether_terminal_text as strip;
