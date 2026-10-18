//! Stream timing and approximate token-throughput measurement.
use crate::StreamingState;
use std::time::Instant;
impl StreamingState {
    /// Restart the clock and character counter without clearing text.
    pub fn begin_streaming(&mut self) {
        self.streaming_start = Some(Instant::now());
        self.streaming_chars = 0;
    }
    /// Add characters to the saturating throughput counter.
    pub fn record_streaming_chars(&mut self, len: usize) {
        self.streaming_chars = self.streaming_chars.saturating_add(len);
    }
    /// Estimate tokens per second after at least 100 milliseconds.
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
