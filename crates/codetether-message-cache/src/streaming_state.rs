//! Streaming text mutations coordinated with rendered-suffix invalidation.
use crate::RenderedLineCache;
use codetether_chat_model::StreamingState;

impl RenderedLineCache {
    /// Replace the cumulative preview and invalidate its rendered suffix.
    /// # Examples
    /// ```
    /// let mut cache = codetether_message_cache::RenderedLineCache::default();
    /// let mut stream = codetether_chat_model::StreamingState::default();
    /// cache.replace_streaming_text(&mut stream, "reply".into());
    /// assert_eq!(stream.streaming_text, "reply");
    /// ```
    pub fn replace_streaming_text(&mut self, streaming: &mut StreamingState, text: String) {
        streaming.replace_streaming_text(text);
        self.invalidate_streaming();
    }

    /// Clear preview text, invalidating its rendered suffix only when it changed.
    /// # Examples
    /// ```
    /// let mut cache = codetether_message_cache::RenderedLineCache::default();
    /// let mut stream = codetether_chat_model::StreamingState::default();
    /// cache.replace_streaming_text(&mut stream, "reply".into());
    /// cache.clear_streaming_text(&mut stream);
    /// assert!(stream.streaming_text.is_empty());
    /// ```
    pub fn clear_streaming_text(&mut self, streaming: &mut StreamingState) {
        if streaming.clear_streaming_text() {
            self.invalidate_streaming();
        }
    }
}
