//! Adaptive limiter reset operations.
use super::*;
impl AdaptiveRateLimiter {
    /// Reset the rate limiter to initial state
    ///
    /// # Returns
    /// Unit after completing the state update or delay.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// limiter.record_retry().await;
    /// limiter.reset().await;
    /// assert_eq!(limiter.get_stats().await.retry_count, 0);
    /// # }
    /// ```
    pub async fn reset(&self) {
        let mut current_delay = self.current_delay_ms.write().await;
        *current_delay = self.base_delay_ms;
        let mut backoff = self.backoff_multiplier.write().await;
        *backoff = 1.0;

        let mut stats = self.stats.write().await;
        *stats = RateLimitStats::default();

        let mut info = self.rate_limit_info.write().await;
        *info = RateLimitInfo::default();

        let mut times = self.request_times.write().await;
        times.clear();
    }
}
