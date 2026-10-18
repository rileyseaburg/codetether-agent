//! Text and thinking stream events.

use crate::StreamView;
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_util::preview_chars::truncate_preview;

/// Apply chat text presentation and timing effects.
/// # Arguments
/// Receives frontend state and the owned event or payload fields.
/// # Returns
/// Updates the supplied frontend state in event order.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_chat_events::StreamView) {
/// let _ = codetether_chat_events::text::chunk(view, String::new());
/// # }
/// ```
pub fn chunk(view: &mut impl StreamView, chunk: String) {
    if view.auto_follow() {
        view.scroll_to_bottom();
    }
    view.note_text_token();
    let chunk_len = chunk.len();
    view.replace_streaming_text(super::bounded::bounded_chunk(
        chunk,
        view.max_streaming_bytes(),
    ));
    view.record_streaming_chars(chunk_len);
    view.set_status(format!("Streaming reply… {} chars", view.streaming_len()));
}

/// Apply chat text presentation and timing effects.
/// # Arguments
/// Receives frontend state and the owned event or payload fields.
/// # Returns
/// Updates the supplied frontend state in event order.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_chat_events::StreamView) {
/// let _ = codetether_chat_events::text::complete(view, String::new());
/// # }
/// ```
pub fn complete(view: &mut impl StreamView, text: String) {
    view.note_text_token();
    view.clear_streaming_text();
    view.messages()
        .push(ChatMessage::new(MessageType::Assistant, text));
    view.set_status("Assistant replied".to_string());
    if view.auto_follow() {
        view.scroll_to_bottom();
    }
}

/// Apply chat text presentation and timing effects.
/// # Arguments
/// Receives frontend state and the owned event or payload fields.
/// # Returns
/// Updates the supplied frontend state in event order.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_chat_events::StreamView) {
/// let _ = codetether_chat_events::text::thinking_complete(view, String::new());
/// # }
/// ```
pub fn thinking_complete(view: &mut impl StreamView, text: String) {
    if text.is_empty() {
        return;
    }
    view.reset_tool_preview_scroll();
    view.set_status(format!(
        "Reasoning: {}",
        truncate_preview(&text.replace('\n', " "), 96)
    ));
    view.messages()
        .push(ChatMessage::new(MessageType::Thinking(text.clone()), text));
    if view.auto_follow() {
        view.scroll_to_bottom();
    }
}
