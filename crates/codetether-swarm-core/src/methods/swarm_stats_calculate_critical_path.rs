//! SwarmStats calculate_critical_path calculation.
use super::*;
impl SwarmStats {
    /// Calculate critical path from stages
    ///
    /// # Returns
    /// Unit after updating the corresponding counters or metric.
    /// # Examples
    /// ```
    /// let mut stats = codetether_swarm_core::SwarmStats::default();
    /// stats.calculate_critical_path();
    /// assert_eq!(stats.critical_path_length, 0);
    /// ```
    pub fn calculate_critical_path(&mut self) {
        self.critical_path_length = self.stages.iter().map(|s| s.max_steps).sum();
    }
}
