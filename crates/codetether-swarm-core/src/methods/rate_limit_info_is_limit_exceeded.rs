//! RateLimitInfo is_limit_exceeded calculation.
use super::*;
impl RateLimitInfo {
    /// Check if we've hit the rate limit
    ///
    /// # Returns
    /// The computed metric or policy value.
    /// # Examples
    /// ```
    /// let info = codetether_swarm_core::RateLimitInfo::default();
    /// assert!(!info.is_limit_exceeded());
    /// ```
    pub fn is_limit_exceeded(&self) -> bool {
        self.remaining == Some(0)
    }
}
