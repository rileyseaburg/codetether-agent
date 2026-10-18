//! Provider-message projection into visible chat rows and plain exports.
//!
//! provider_messages_to_chat_messages preserves message-part ordering, hides
//! internal context markers, and correlates tool result names across messages.
//! extract_message_text provides an unstructured text export of content parts.
mod attachments;
mod buffer;
mod dispatch;
mod hidden;
mod parts;
mod text;
mod thinking;
mod tool_call;
mod tool_result;
mod transcript;
use parts::PartProjection;
pub use text::extract_message_text;
pub use transcript::provider_messages_to_chat_messages;
#[cfg(test)]
mod text_tests;
