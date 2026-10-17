//! Adaptive shared rate limiter using asynchronous locks and bounded delays.
//!
//! Construct AdaptiveRateLimiter, record responses, and wait before subsequent
//! requests. Clones share counters, header state and adaptive delay.
use crate::{RateLimitInfo, RateLimitStats};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
mod construct;
mod delay;
mod observation;
mod rate_limit;
mod reset;
mod success;
mod success_headers;
#[cfg(test)]
mod tests;

/// Adaptive rate limiter that adjusts based on provider responses
// TODO: fold backoff/current_rpm into stats struct for single-lock atomicity
#[derive(Debug, Clone)]
///
/// # Examples
/// ```no_run
/// # async fn example() {
/// let limiter = codetether_swarm_core::rate_limiter::AdaptiveRateLimiter::new(1000);
/// assert_eq!(limiter.current_delay().await, std::time::Duration::from_millis(1000));
/// # }
/// ```
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
