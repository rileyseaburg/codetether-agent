//! Bounded backoff configuration for whole-request stream restarts.

use std::time::Duration;

use codetether_stream_outcome::StreamStop;

/// Backoff and retry limits for whole-request stream recovery.
/// # Examples
/// ```
/// let policy = codetether_stream_restart::RestartPolicy::default();
/// assert_eq!(policy.max_restarts, 5);
/// ```
#[derive(Debug, Clone)]
pub struct RestartPolicy {
    /// Maximum retries before transport fallback or terminal failure.
    pub max_restarts: u32,
    /// Initial delay before exponential jitter is applied.
    pub base_backoff: Duration,
    /// Exponential backoff multiplier.
    pub multiplier: u32,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            max_restarts: 5,
            base_backoff: Duration::from_millis(200),
            multiplier: 2,
        }
    }
}

impl RestartPolicy {
    /// Compute the shared provider jittered delay for this attempt.
    pub fn backoff(&self, attempt: u32) -> Duration {
        codetether_provider_core::retry::timing::jittered(
            self.base_backoff,
            self.multiplier,
            attempt,
        )
    }

    /// Prefer provider retry guidance; otherwise use jittered backoff.
    pub fn delay(&self, attempt: u32, stop: &StreamStop) -> Duration {
        let suggested = match stop {
            StreamStop::Fault { message, .. } => {
                codetether_provider_core::retry::timing::from_message(message)
            }
            _ => None,
        };
        suggested.unwrap_or_else(|| self.backoff(attempt))
    }
}
