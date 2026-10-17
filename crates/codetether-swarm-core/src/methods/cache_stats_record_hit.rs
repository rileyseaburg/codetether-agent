//! CacheStats record_hit calculation.
use super::*;
impl CacheStats {
    /// Record a cache hit
    ///
    /// # Returns
    /// Unit after updating the corresponding counters or metric.
    /// # Examples
    /// ```
    /// let mut stats = codetether_swarm_core::CacheStats::default();
    /// stats.record_hit();
    /// assert_eq!(stats.hits, 1);
    /// ```
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }
}
