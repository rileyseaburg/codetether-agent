//! CacheStats total_lookups calculation.
use super::*;
impl CacheStats {
    /// Total number of cache lookups
    ///
    /// # Returns
    /// The computed metric or policy value.
    /// # Examples
    /// ```
    /// let stats = codetether_swarm_core::CacheStats::default();
    /// assert_eq!(stats.total_lookups(), 0);
    /// ```
    pub fn total_lookups(&self) -> u64 {
        self.hits + self.misses
    }
}
