//! Stream-state, timing and message presentation bindings.
use codetether_chat_model::ChatMessage;
/// Frontend state and effects used by text and retry events.
/// # Implementors
/// Frontends retain timing and stream state while exposing these operations.
pub trait StreamView {
    /// Whether new text should remain visible at the bottom.
    fn auto_follow(&self) -> bool;
    /// Scroll to the latest chat content.
    fn scroll_to_bottom(&mut self);
    /// Record the arrival time of a text token.
    fn note_text_token(&mut self);
    /// Replace the accumulated stream preview.
    fn replace_streaming_text(&mut self, text: String);
    /// Record the size of the incoming cumulative text.
    fn record_streaming_chars(&mut self, count: usize);
    /// Current preview length in bytes.
    fn streaming_len(&self) -> usize;
    /// Maximum bytes retained for a stream preview.
    fn max_streaming_bytes(&self) -> usize;
    /// Clear the active stream preview.
    fn clear_streaming_text(&mut self);
    /// Reset tool-preview scrolling before showing reasoning.
    fn reset_tool_preview_scroll(&mut self);
    /// Borrow chronological chat messages.
    fn messages(&mut self) -> &mut Vec<ChatMessage>;
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
}
