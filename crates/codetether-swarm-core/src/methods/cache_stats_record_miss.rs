//! CacheStats record_miss calculation.
use super::*;
impl CacheStats {
    /// Record a cache miss
    ///
    /// # Returns
    /// Unit after updating the corresponding counters or metric.
    /// # Examples
    /// ```
    /// let mut stats = codetether_swarm_core::CacheStats::default();
    /// stats.record_miss();
    /// assert_eq!(stats.misses, 1);
    /// ```
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }
}
