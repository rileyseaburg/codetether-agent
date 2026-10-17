//! SwarmStats cache_hit_rate calculation.
use super::*;
impl SwarmStats {
    /// Cache hit rate (0.0 to 1.0); returns 0.0 when no cache lookups occurred
    ///
    /// # Returns
    /// The computed metric or policy value.
    /// # Examples
    /// ```
    /// let stats = codetether_swarm_core::SwarmStats::default();
    /// assert_eq!(stats.cache_hit_rate(), 0.0);
    /// ```
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}
