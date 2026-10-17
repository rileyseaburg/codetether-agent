//! RateLimitInfo from_headers calculation.
use super::*;
impl RateLimitInfo {
    /// Parse rate limit headers from a header map
    ///
    /// # Arguments
    /// * `headers` - Provider response header names and values.
    /// # Returns
    /// The computed metric or policy value.
    /// # Examples
    /// ```
    /// let info = codetether_swarm_core::RateLimitInfo::from_headers(&std::collections::HashMap::new());
    /// assert_eq!(info.remaining, None);
    /// ```
    pub fn from_headers(headers: &HashMap<String, String>) -> Self {
        let mut info = Self::default();

        for (key, value) in headers {
            let key_lower = key.to_lowercase();
            match key_lower.as_str() {
                // X-RateLimit-Remaining or x-ratelimit-remaining
                k if k.contains("ratelimit-remaining") => {
                    info.remaining = value.parse().ok();
                }
                // X-RateLimit-Limit or x-ratelimit-limit
                k if k.contains("ratelimit-limit") => {
                    info.limit = value.parse().ok();
                }
                // X-RateLimit-Reset or x-ratelimit-reset (timestamp)
                k if k.contains("ratelimit-reset") && !k.contains("after") => {
                    info.reset_at = value.parse().ok();
                }
                // X-RateLimit-Reset-After or x-ratelimit-reset-after (seconds)
                k if k.contains("ratelimit-reset-after") => {
                    info.reset_after_secs = value.parse().ok();
                }
                // Retry-After
                "retry-after" => {
                    info.retry_after_secs = value.parse().ok();
                }
                // X-RateLimit-Policy
                k if k.contains("ratelimit-policy") => {
                    info.policy = Some(value.clone());
                }
                _ => {}
            }
        }

        info
    }
}
