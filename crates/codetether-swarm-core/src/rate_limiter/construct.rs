//! Construct the original shared lock state and default delay.
use super::*;
impl AdaptiveRateLimiter {
    /// Create a new adaptive rate limiter
    ///
    /// # Arguments
    /// * `base_delay_ms` - Initial adaptive delay in milliseconds.
    /// # Returns
    /// A limiter with shared lock-backed state.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// assert_eq!(limiter.current_delay().await, std::time::Duration::from_millis(1000));
    /// # }
    /// ```
    pub fn new(base_delay_ms: u64) -> Self {
        Self {
            base_delay_ms,
            min_delay_ms: 100,    // 100ms minimum
            max_delay_ms: 60_000, // 60s maximum
            current_delay_ms: Arc::new(RwLock::new(base_delay_ms)),
            backoff_multiplier: Arc::new(RwLock::new(1.0)),
            stats: Arc::new(RwLock::new(RateLimitStats::default())),
            rate_limit_info: Arc::new(RwLock::new(RateLimitInfo::default())),
            request_times: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for AdaptiveRateLimiter {
    fn default() -> Self {
        Self::new(1000) // 1 second default base delay
    }
}
