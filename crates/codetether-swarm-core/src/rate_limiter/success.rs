//! Record success, decay backoff and update the rolling request rate.
use super::*;
impl AdaptiveRateLimiter {
    /// Record a successful request with optional rate limit headers
    ///
    /// # Arguments
    /// * `headers` - Optional response headers for rate-limit updates.
    /// # Returns
    /// Unit after completing the state update or delay.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
    /// limiter.record_success(None).await;
    /// assert_eq!(limiter.get_stats().await.total_requests, 1);
    /// # }
    /// ```
    pub async fn record_success(&self, headers: Option<&HashMap<String, String>>) {
        let now = Instant::now();
        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.apply_success_headers(headers).await;

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        stats.last_request_timestamp_secs = Some(now_secs);
        let current_delay = *self.current_delay_ms.read().await;
        stats.adaptive_delay_ms = current_delay;

        // Reset backoff multiplier on success
        let mut backoff = self.backoff_multiplier.write().await;
        *backoff = 1.0_f64.max(*backoff * 0.9);

        // Update request times for RPM calculation
        drop(stats);
        let mut times = self.request_times.write().await;
        times.push(now);

        // Keep only last 60 seconds of request times
        let cutoff = now - Duration::from_secs(60);
        times.retain(|&t| t > cutoff);

        // Update RPM in stats
        let rpm = times.len() as f64;
        let mut stats = self.stats.write().await;
        stats.current_rpm = rpm;
    }
}
