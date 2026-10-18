//! Application cache invalidation around shared streaming state.
impl super::AppState {
    pub fn begin_streaming(&mut self) {
        self.streaming.begin_streaming();
    }
    pub fn record_streaming_chars(&mut self, len: usize) {
        self.streaming.record_streaming_chars(len);
    }
    /// Replace cumulative reply text and invalidate its rendered suffix.
    pub fn replace_streaming_text(&mut self, text: String) {
        self.streaming.replace_streaming_text(text);
        self.message_cache.invalidate_streaming();
    }
    /// Clear reply text and invalidate its rendered suffix only when it changed.
    pub fn clear_streaming_text(&mut self) {
        if self.streaming.clear_streaming_text() {
            self.message_cache.invalidate_streaming();
        }
    }
    pub fn streaming_tok_per_sec(&self) -> Option<f64> {
        self.streaming.streaming_tok_per_sec()
    }
}
