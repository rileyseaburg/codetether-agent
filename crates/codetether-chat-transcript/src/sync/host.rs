//! Application presentation operations for transcript replacement.
use super::Metrics;
use codetether_chat_model::ChatMessage;
use codetether_provider_core::Message;
/// Implement this on the native chat view to synchronize a loaded transcript.
pub trait View {
    /// Replace the chronological visible chat buffer.
    fn replace_messages(&mut self, messages: Vec<ChatMessage>);
    /// Initialize history pagination with the visible prefix boundary.
    fn reset_history(
        &mut self,
        source_id: String,
        boundary: &[Message],
        depth: usize,
        has_older: bool,
    );
    /// Pin the absolute message offset before the visible history.
    fn set_history_before(&mut self, before: usize);
    /// Invalidate cached rendered messages.
    fn clear_cache(&mut self);
    /// Borrow the metrics cleared when changing transcripts.
    fn metrics(&mut self) -> Metrics<'_>;
    /// Reset the tool-preview scroll position.
    fn reset_tool_preview_scroll(&mut self);
    /// Replace the tool-preview scroll extent.
    fn set_tool_preview_max_scroll(&mut self, max: usize);
    /// Scroll chat to its latest message.
    fn scroll_to_bottom(&mut self);
}
