//! Streaming throughput tracking methods.
//!
//! Tracks streaming start time and character counts to estimate
//! tokens-per-second for the UI.

use std::time::Instant;

impl super::AppState {
    pub fn begin_streaming(&mut self) {
        self.streaming_start = Some(Instant::now());
        self.streaming_chars = 0;
    }

    pub fn record_streaming_chars(&mut self, len: usize) {
        self.streaming_chars = self.streaming_chars.saturating_add(len);
    }

    /// Replace the cumulative preview and invalidate only its rendered suffix.
    pub fn replace_streaming_text(&mut self, text: String) {
        self.streaming_text = text;
        self.cached_streaming_current = false;
    }

    /// Clear the cumulative preview and invalidate its rendered suffix.
    pub fn clear_streaming_text(&mut self) {
        if !self.streaming_text.is_empty() {
            self.streaming_text.clear();
            self.cached_streaming_current = false;
        }
    }

    pub fn streaming_tok_per_sec(&self) -> Option<f64> {
        let start = self.streaming_start?;
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed < 0.1 {
            return None;
        }
        let tokens = self.streaming_chars as f64 / 4.0;
        Some(tokens / elapsed)
    }
}
