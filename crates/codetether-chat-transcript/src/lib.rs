//! Provider-message projection into visible chat rows and plain exports.
//!
//! provider_messages_to_chat_messages preserves message-part ordering, hides
//! internal context markers, and correlates tool result names across messages.
//! extract_message_text provides an unstructured text export of content parts.
//! build_transcript exports conversation turns; clipboard_text formats a raw row.
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

mod export;
pub use export::build_transcript;
#[cfg(test)]
mod transcript_empty_for_empty_history;
#[cfg(test)]
mod transcript_has_no_render_artifacts;
#[cfg(test)]
mod transcript_joins_user_and_assistant_turns;
#[cfg(test)]
mod transcript_skips_empty_content;
#[cfg(test)]
mod transcript_skips_internal_message_types;

mod clipboard;
pub use clipboard::clipboard_text;
#[cfg(test)]
mod assistant_clipboard_text_has_no_render_artifacts;
#[cfg(test)]
mod thinking_clipboard_text_uses_inner_text;

/// Loaded-transcript projection, pagination initialization and metric reset.
pub mod sync;
