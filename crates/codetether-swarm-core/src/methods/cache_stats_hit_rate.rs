//! CacheStats hit_rate calculation.
use super::*;
impl CacheStats {
    /// Cache hit rate (0.0 to 1.0)
    ///
    /// # Returns
    /// The computed metric or policy value.
    /// # Examples
    /// ```
    /// let stats = codetether_swarm_core::CacheStats::default();
    /// assert_eq!(stats.hit_rate(), 0.0);
    /// ```
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_lookups();
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}
