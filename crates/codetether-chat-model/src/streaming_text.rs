//! Replacing and clearing the cumulative assistant preview.
use crate::StreamingState;
impl StreamingState {
    /// Replace the cumulative preview text.
    pub fn replace_streaming_text(&mut self, text: String) {
        self.streaming_text = text;
    }
    /// Clear reply text and report whether any content changed.
    pub fn clear_streaming_text(&mut self) -> bool {
        if !self.streaming_text.is_empty() {
            self.streaming_text.clear();
            true
        } else {
            false
        }
    }
}
