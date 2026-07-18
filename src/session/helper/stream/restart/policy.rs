//! Bounded backoff configuration for whole-request stream restarts.

use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) struct RestartPolicy {
    pub(crate) max_restarts: u32,
    pub(crate) base_backoff: Duration,
    pub(crate) multiplier: u32,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            max_restarts: 3,
            base_backoff: Duration::from_secs(2),
            multiplier: 2,
        }
    }
}

impl RestartPolicy {
    pub(crate) fn backoff(&self, attempt: u32) -> Duration {
        self.base_backoff * self.multiplier.saturating_pow(attempt)
    }
}
