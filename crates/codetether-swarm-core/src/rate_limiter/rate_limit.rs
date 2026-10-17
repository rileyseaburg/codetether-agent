//! Adaptive limiter rate_limit operations.
use super::*;
impl AdaptiveRateLimiter {
    /// Record a rate limit hit (429 response)
    ///
    /// # Arguments
    /// * `retry_after` - Optional delay in seconds supplied by the provider.
    /// # Returns
    /// Unit after completing the state update or delay.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// limiter.record_rate_limit_hit(None).await;
    /// assert_eq!(limiter.get_stats().await.rate_limit_hits, 1);
    /// # }
    /// ```
    pub async fn record_rate_limit_hit(&self, retry_after: Option<u64>) {
        let mut stats = self.stats.write().await;
        stats.rate_limit_hits += 1;
        stats.retry_count += 1;
        drop(stats);

        // Increase backoff multiplier exponentially
        let mut backoff = self.backoff_multiplier.write().await;
        *backoff = (*backoff * 2.0).min(32.0);
        let current_backoff = *backoff;
        drop(backoff);

        // Set delay based on retry-after or exponential backoff
        let delay_ms = if let Some(retry) = retry_after {
            retry * 1000
        } else {
            let base = self.base_delay_ms as f64;
            (base * current_backoff) as u64
        };

        let mut current_delay = self.current_delay_ms.write().await;
        *current_delay = delay_ms.min(self.max_delay_ms);
        let delay_value = *current_delay;
        drop(current_delay);

        let mut stats = self.stats.write().await;
        stats.adaptive_delay_ms = delay_value;
    }
}
