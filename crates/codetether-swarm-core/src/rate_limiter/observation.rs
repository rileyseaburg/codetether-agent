//! Adaptive limiter observation operations.
use super::*;
impl AdaptiveRateLimiter {
    /// Record a retry attempt
    ///
    /// # Returns
    /// Unit after completing the state update or delay.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// limiter.record_retry().await;
    /// assert_eq!(limiter.get_stats().await.retry_count, 1);
    /// # }
    /// ```
    pub async fn record_retry(&self) {
        let mut stats = self.stats.write().await;
        stats.retry_count += 1;
    }

    /// Get current statistics
    ///
    /// # Returns
    /// A snapshot of the requested state.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// assert_eq!(limiter.get_stats().await.total_requests, 0);
    /// # }
    /// ```
    pub async fn get_stats(&self) -> RateLimitStats {
        self.stats.read().await.clone()
    }

    /// Get current rate limit info
    ///
    /// # Returns
    /// A snapshot of the requested state.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// assert_eq!(limiter.get_rate_limit_info().await.remaining, None);
    /// # }
    /// ```
    pub async fn get_rate_limit_info(&self) -> RateLimitInfo {
        self.rate_limit_info.read().await.clone()
    }
}
