//! RateLimitInfo is_approaching_limit calculation.
use super::*;
impl RateLimitInfo {
    /// Check if we're approaching the rate limit (less than 20% remaining)
    ///
    /// # Returns
    /// The computed metric or policy value.
    /// # Examples
    /// ```
    /// let info = codetether_swarm_core::RateLimitInfo::default();
    /// assert!(!info.is_approaching_limit());
    /// ```
    pub fn is_approaching_limit(&self) -> bool {
        match (self.remaining, self.limit) {
            (Some(remaining), Some(limit)) if limit > 0 => (remaining as f64 / limit as f64) < 0.2,
            _ => false,
        }
    }
}
