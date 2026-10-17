//! RateLimitInfo recommended_delay calculation.
use super::*;
impl RateLimitInfo {
    /// Calculate recommended delay before next request
    ///
    /// # Returns
    /// The computed metric or policy value.
    /// # Examples
    /// ```
    /// let info = codetether_swarm_core::RateLimitInfo::default();
    /// assert_eq!(info.recommended_delay(), std::time::Duration::ZERO);
    /// ```
    pub fn recommended_delay(&self) -> Duration {
        // If we have a retry-after header, use that
        if let Some(retry_after) = self.retry_after_secs {
            return Duration::from_secs(retry_after);
        }

        // If we're approaching limit and have reset info, calculate delay
        if self.is_approaching_limit() {
            if let Some(reset_after) = self.reset_after_secs {
                // Spread requests evenly across remaining time
                if let Some(remaining) = self.remaining
                    && remaining > 0
                {
                    return Duration::from_millis((reset_after * 1000) / remaining as u64);
                }
                return Duration::from_secs(reset_after);
            }

            if let Some(reset_at) = self.reset_at {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                if reset_at > now {
                    let remaining_secs = reset_at - now;
                    if let Some(remaining) = self.remaining
                        && remaining > 0
                    {
                        return Duration::from_millis((remaining_secs * 1000) / remaining as u64);
                    }
                    return Duration::from_secs(remaining_secs);
                }
            }
        }

        Duration::from_millis(0)
    }
}
