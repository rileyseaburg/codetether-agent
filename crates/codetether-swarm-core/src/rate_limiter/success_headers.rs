//! Apply successful-response headers while preserving their lock scope.
use super::*;
impl AdaptiveRateLimiter {
    pub(super) async fn apply_success_headers(&self, headers: Option<&HashMap<String, String>>) {
        // Update rate limit info if headers provided
        if let Some(h) = headers {
            let info = RateLimitInfo::from_headers(h);
            let mut rate_limit = self.rate_limit_info.write().await;
            *rate_limit = info.clone();

            // Adjust delay based on remaining requests
            if let (Some(remaining), Some(limit)) = (info.remaining, info.limit)
                && limit > 0
            {
                let ratio = remaining as f64 / limit as f64;
                let backoff = *self.backoff_multiplier.read().await;
                let mut new_delay = self.base_delay_ms as f64;

                if ratio < 0.1 {
                    // Less than 10% remaining - be very conservative
                    new_delay *= 3.0;
                } else if ratio < 0.3 {
                    // Less than 30% remaining - be cautious
                    new_delay *= 1.5;
                } else if ratio > 0.5 && backoff <= 1.0 {
                    // More than 50% remaining and no recent 429s - can be more aggressive
                    new_delay *= 0.8;
                }

                let mut current_delay = self.current_delay_ms.write().await;
                *current_delay = new_delay as u64;
            }
        }
    }
}
