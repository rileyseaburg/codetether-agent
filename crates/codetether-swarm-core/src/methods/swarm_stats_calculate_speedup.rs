//! SwarmStats calculate_speedup calculation.
use super::*;
impl SwarmStats {
    /// Calculate the speedup factor
    ///
    /// # Returns
    /// Unit after updating the corresponding counters or metric.
    /// # Examples
    /// ```
    /// let mut stats = codetether_swarm_core::SwarmStats { execution_time_ms: 10, sequential_time_estimate_ms: 20, ..Default::default() };
    /// stats.calculate_speedup();
    /// assert_eq!(stats.speedup_factor, 2.0);
    /// ```
    pub fn calculate_speedup(&mut self) {
        if self.execution_time_ms > 0 {
            self.speedup_factor =
                self.sequential_time_estimate_ms as f64 / self.execution_time_ms as f64;
        }
    }
}
