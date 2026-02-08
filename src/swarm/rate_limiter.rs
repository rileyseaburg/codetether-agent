//! Adaptive rate limiting for swarm operations
//!
//! Parses rate limit headers from provider responses and dynamically
//! adjusts request timing to avoid hitting rate limits.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Rate limit information extracted from API response headers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Number of requests remaining in the current window
    pub remaining: Option<u32>,
    /// Total limit for the current window
    pub limit: Option<u32>,
    /// Seconds until the rate limit resets
    pub reset_after_secs: Option<u64>,
    /// Unix timestamp when the rate limit resets
    pub reset_at: Option<u64>,
    /// Retry-After header value (seconds to wait before retry)
    pub retry_after_secs: Option<u64>,
    /// The policy string (e.g., "60:100" for 60 seconds, 100 requests)
    pub policy: Option<String>,
}

impl RateLimitInfo {
    /// Parse rate limit headers from a header map
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

    /// Check if we're approaching the rate limit (less than 20% remaining)
    pub fn is_approaching_limit(&self) -> bool {
        match (self.remaining, self.limit) {
            (Some(remaining), Some(limit)) if limit > 0 => {
                (remaining as f64 / limit as f64) < 0.2
            }
            _ => false,
        }
    }

    /// Check if we've hit the rate limit
    pub fn is_limit_exceeded(&self) -> bool {
        self.remaining == Some(0)
    }

    /// Calculate recommended delay before next request
    pub fn recommended_delay(&self) -> Duration {
        // If we have a retry-after header, use that
        if let Some(retry_after) = self.retry_after_secs {
            return Duration::from_secs(retry_after);
        }

        // If we're approaching limit and have reset info, calculate delay
        if self.is_approaching_limit() {
            if let Some(reset_after) = self.reset_after_secs {
                // Spread requests evenly across remaining time
                if let Some(remaining) = self.remaining {
                    if remaining > 0 {
                        return Duration::from_millis((reset_after * 1000) / remaining as u64);
                    }
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
                    if let Some(remaining) = self.remaining {
                        if remaining > 0 {
                            return Duration::from_millis((remaining_secs * 1000) / remaining as u64);
                        }
                    }
                    return Duration::from_secs(remaining_secs);
                }
            }
        }

        Duration::from_millis(0)
    }
}

/// Statistics for rate limit tracking
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimitStats {
    /// Total number of requests made
    pub total_requests: u64,
    /// Number of 429 responses received
    pub rate_limit_hits: u64,
    /// Number of retries due to rate limiting
    pub retry_count: u64,
    /// Current estimated requests per minute
    pub current_rpm: f64,
    /// Average delay between requests (ms)
    pub avg_delay_ms: u64,
    /// Current adaptive delay (ms)
    pub adaptive_delay_ms: u64,
    /// Last rate limit info received
    pub last_rate_limit_info: Option<RateLimitInfo>,
    /// Unix timestamp of last request (seconds since epoch)
    pub last_request_timestamp_secs: Option<u64>,
}

/// Adaptive rate limiter that adjusts based on provider responses
#[derive(Debug, Clone)]
pub struct AdaptiveRateLimiter {
    /// Base delay between requests
    base_delay_ms: u64,
    /// Minimum delay (ms)
    min_delay_ms: u64,
    /// Maximum delay (ms)
    max_delay_ms: u64,
    /// Current adaptive delay
    current_delay_ms: Arc<RwLock<u64>>,
    /// Exponential backoff multiplier for 429s
    backoff_multiplier: Arc<RwLock<f64>>,
    /// Statistics
    stats: Arc<RwLock<RateLimitStats>>,
    /// Last rate limit info
    rate_limit_info: Arc<RwLock<RateLimitInfo>>,
    /// Request timestamps for RPM calculation
    request_times: Arc<RwLock<Vec<Instant>>>,
}

impl AdaptiveRateLimiter {
    /// Create a new adaptive rate limiter
    pub fn new(base_delay_ms: u64) -> Self {
        Self {
            base_delay_ms,
            min_delay_ms: 100,      // 100ms minimum
            max_delay_ms: 60_000,   // 60s maximum
            current_delay_ms: Arc::new(RwLock::new(base_delay_ms)),
            backoff_multiplier: Arc::new(RwLock::new(1.0)),
            stats: Arc::new(RwLock::new(RateLimitStats::default())),
            rate_limit_info: Arc::new(RwLock::new(RateLimitInfo::default())),
            request_times: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get the current delay
    pub async fn current_delay(&self) -> Duration {
        let info = self.rate_limit_info.read().await;
        let recommended = info.recommended_delay();

        // Use the larger of adaptive delay or recommended delay
        let current = *self.current_delay_ms.read().await;
        let delay_ms = current.max(recommended.as_millis() as u64);
        Duration::from_millis(delay_ms.min(self.max_delay_ms).max(self.min_delay_ms))
    }

    /// Record a successful request with optional rate limit headers
    pub async fn record_success(&self, headers: Option<&HashMap<String, String>>) {
        let now = Instant::now();
        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Update rate limit info if headers provided
        if let Some(h) = headers {
            let info = RateLimitInfo::from_headers(h);
            let mut rate_limit = self.rate_limit_info.write().await;
            *rate_limit = info.clone();

            // Adjust delay based on remaining requests
            if let (Some(remaining), Some(limit)) = (info.remaining, info.limit) {
                if limit > 0 {
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

    /// Record a rate limit hit (429 response)
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

    /// Record a retry attempt
    pub async fn record_retry(&self) {
        let mut stats = self.stats.write().await;
        stats.retry_count += 1;
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> RateLimitStats {
        self.stats.read().await.clone()
    }

    /// Get current rate limit info
    pub async fn get_rate_limit_info(&self) -> RateLimitInfo {
        self.rate_limit_info.read().await.clone()
    }

    /// Wait for the appropriate delay before making next request
    pub async fn wait(&self) {
        let delay = self.current_delay().await;
        if delay > Duration::from_millis(0) {
            tokio::time::sleep(delay).await;
        }
    }

    /// Reset the rate limiter to initial state
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

impl Default for AdaptiveRateLimiter {
    fn default() -> Self {
        Self::new(1000) // 1 second default base delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_info_from_headers() {
        let mut headers = HashMap::new();
        headers.insert("x-ratelimit-remaining".to_string(), "45".to_string());
        headers.insert("x-ratelimit-limit".to_string(), "100".to_string());
        headers.insert("x-ratelimit-reset-after".to_string(), "60".to_string());
        headers.insert("retry-after".to_string(), "5".to_string());

        let info = RateLimitInfo::from_headers(&headers);

        assert_eq!(info.remaining, Some(45));
        assert_eq!(info.limit, Some(100));
        assert_eq!(info.reset_after_secs, Some(60));
        assert_eq!(info.retry_after_secs, Some(5));
    }

    #[test]
    fn test_is_approaching_limit() {
        let info = RateLimitInfo {
            remaining: Some(15),
            limit: Some(100),
            ..Default::default()
        };
        assert!(info.is_approaching_limit());

        let info2 = RateLimitInfo {
            remaining: Some(50),
            limit: Some(100),
            ..Default::default()
        };
        assert!(!info2.is_approaching_limit());
    }

    #[test]
    fn test_recommended_delay_with_retry_after() {
        let info = RateLimitInfo {
            retry_after_secs: Some(10),
            ..Default::default()
        };
        assert_eq!(info.recommended_delay(), Duration::from_secs(10));
    }

    #[test]
    fn test_recommended_delay_when_approaching_limit() {
        let info = RateLimitInfo {
            remaining: Some(5),
            limit: Some(100),
            reset_after_secs: Some(60),
            ..Default::default()
        };
        // Should spread 5 requests over 60 seconds = 12 seconds each
        assert_eq!(info.recommended_delay(), Duration::from_secs(12));
    }
}