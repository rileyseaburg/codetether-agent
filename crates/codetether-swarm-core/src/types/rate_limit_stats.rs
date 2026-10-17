//! RateLimitStats shared contract.
use super::*;

/// Statistics for rate limit tracking
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let stats = codetether_swarm_core::RateLimitStats::default();
/// assert_eq!(stats.total_requests, 0);
/// ```
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
