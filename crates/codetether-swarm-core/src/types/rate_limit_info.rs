//! RateLimitInfo shared contract.
use super::*;

/// Rate limit information extracted from API response headers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let info = codetether_swarm_core::RateLimitInfo::default();
/// assert!(!info.is_limit_exceeded());
/// ```
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
