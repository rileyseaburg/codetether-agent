//! Nearest-rank P95 for the bounded end-to-end latency window.
use super::ChatLatencyStats;
impl ChatLatencyStats {
    /// Compute the nearest-rank 95th percentile end-to-end duration.
    /// # Arguments
    /// None.
    /// # Returns
    /// P95, falling back to the maximum for fewer than three samples.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// assert_eq!(ChatLatencyStats::default().p95_e2e_ms(), None);
    /// ```
    pub fn p95_e2e_ms(&self) -> Option<u64> {
        if self.samples.len() < 3 {
            return self.max_e2e_ms();
        }
        let mut v: Vec<u64> = self.samples.iter().map(|s| s.e2e_ms).collect();
        v.sort_unstable();
        let i = (v.len() as f64 * 0.95).ceil() as usize - 1;
        Some(v[i.min(v.len() - 1)])
    }
}
