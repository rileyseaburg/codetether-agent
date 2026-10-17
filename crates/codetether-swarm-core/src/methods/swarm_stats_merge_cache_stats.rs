//! SwarmStats merge_cache_stats calculation.
use super::*;
impl SwarmStats {
    /// Merge cache statistics from a [`CacheStats`] snapshot into these stats
    ///
    /// # Arguments
    /// * `cache_stats` - Snapshot whose hit and miss counters replace these values.
    /// # Returns
    /// Unit after updating the corresponding counters or metric.
    /// # Examples
    /// ```
    /// let mut stats = codetether_swarm_core::SwarmStats::default();
    /// stats.merge_cache_stats(&codetether_swarm_core::CacheStats { hits: 2, ..Default::default() });
    /// assert_eq!(stats.cache_hits, 2);
    /// ```
    pub fn merge_cache_stats(&mut self, cache_stats: &CacheStats) {
        self.cache_hits = cache_stats.hits;
        self.cache_misses = cache_stats.misses;
    }
}
