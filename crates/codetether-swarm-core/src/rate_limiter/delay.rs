//! Adaptive limiter delay operations.
use super::*;
impl AdaptiveRateLimiter {
    /// Get the current delay
    ///
    /// # Returns
    /// A snapshot of the requested state.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// assert_eq!(limiter.current_delay().await, std::time::Duration::from_millis(1000));
    /// # }
    /// ```
    pub async fn current_delay(&self) -> Duration {
        let info = self.rate_limit_info.read().await;
        let recommended = info.recommended_delay();

        // Use the larger of adaptive delay or recommended delay
        let current = *self.current_delay_ms.read().await;
        let delay_ms = current.max(recommended.as_millis() as u64);
        Duration::from_millis(delay_ms.min(self.max_delay_ms).max(self.min_delay_ms))
    }

    /// Wait for the appropriate delay before making next request
    ///
    /// # Returns
    /// Unit after completing the state update or delay.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// limiter.wait().await;
    /// # }
    /// ```
    pub async fn wait(&self) {
        let delay = self.current_delay().await;
        if delay > Duration::from_millis(0) {
            tokio::time::sleep(delay).await;
        }
    }
}
