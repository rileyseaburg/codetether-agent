//! Record completed turns and reset the bounded sample window.
use super::{ChatLatencyStats, TurnSample, WINDOW};
impl ChatLatencyStats {
    /// Record a completed turn.
    /// Record a completed turn, evicting the oldest sample at capacity.
    /// # Arguments
    /// * `e2e_ms` - End-to-end duration in milliseconds.
    /// * `ttft_ms` - Optional time to first token in milliseconds.
    /// # Returns
    /// Unit after recording the sample.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// let mut stats = ChatLatencyStats::default(); stats.record(1200, Some(100)); assert_eq!(stats.count(), 1);
    /// ```
    pub fn record(&mut self, e2e_ms: u64, ttft_ms: Option<u64>) {
        if self.samples.len() >= WINDOW {
            self.samples.remove(0);
        }
        self.samples.push(TurnSample { e2e_ms, ttft_ms });
    }

    /// Remove all timing samples.
    /// # Arguments
    /// None.
    /// # Returns
    /// Unit after clearing the window.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// let mut stats = ChatLatencyStats::default(); stats.record(1, None); stats.clear(); assert_eq!(stats.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}
