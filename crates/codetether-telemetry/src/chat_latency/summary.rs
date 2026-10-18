//! Counts, recent timing, extrema and integer averages.
use super::ChatLatencyStats;
impl ChatLatencyStats {
    /// Count the retained completed turns.
    /// # Arguments
    /// None.
    /// # Returns
    /// The sample count, at most 100.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// assert_eq!(ChatLatencyStats::default().count(), 0);
    /// ```
    pub fn count(&self) -> usize {
        self.samples.len()
    }

    /// Read the most recent end-to-end timing.
    /// # Arguments
    /// None.
    /// # Returns
    /// The last timing, or None for an empty window.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// assert_eq!(ChatLatencyStats::default().last_e2e_ms(), None);
    /// ```
    pub fn last_e2e_ms(&self) -> Option<u64> {
        self.samples.last().map(|s| s.e2e_ms)
    }

    /// Compute the integer mean end-to-end duration.
    /// # Arguments
    /// None.
    /// # Returns
    /// The mean timing, or None for an empty window.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// assert_eq!(ChatLatencyStats::default().avg_e2e_ms(), None);
    /// ```
    pub fn avg_e2e_ms(&self) -> Option<u64> {
        (!self.samples.is_empty())
            .then(|| self.samples.iter().map(|s| s.e2e_ms).sum::<u64>() / self.samples.len() as u64)
    }

    /// Read the minimum end-to-end duration.
    /// # Arguments
    /// None.
    /// # Returns
    /// The minimum timing, or None for an empty window.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// assert_eq!(ChatLatencyStats::default().min_e2e_ms(), None);
    /// ```
    pub fn min_e2e_ms(&self) -> Option<u64> {
        self.samples.iter().map(|s| s.e2e_ms).min()
    }

    /// Read the maximum end-to-end duration.
    /// # Arguments
    /// None.
    /// # Returns
    /// The maximum timing, or None for an empty window.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// assert_eq!(ChatLatencyStats::default().max_e2e_ms(), None);
    /// ```
    pub fn max_e2e_ms(&self) -> Option<u64> {
        self.samples.iter().map(|s| s.e2e_ms).max()
    }

    /// Compute the integer mean of available first-token timings.
    /// # Arguments
    /// None.
    /// # Returns
    /// The mean timing, or None if no samples contain a first-token duration.
    /// # Examples
    /// ```
    /// use codetether_telemetry::chat_latency::ChatLatencyStats;
    /// assert_eq!(ChatLatencyStats::default().avg_ttft_ms(), None);
    /// ```
    pub fn avg_ttft_ms(&self) -> Option<u64> {
        let t: Vec<u64> = self.samples.iter().filter_map(|s| s.ttft_ms).collect();
        (!t.is_empty()).then(|| t.iter().sum::<u64>() / t.len() as u64)
    }
}
