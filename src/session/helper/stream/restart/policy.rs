//! Bounded backoff configuration for whole-request stream restarts.

use std::time::Duration;

use super::super::outcome::StreamStop;

#[derive(Debug, Clone)]
pub(crate) struct RestartPolicy {
    pub(crate) max_restarts: u32,
    pub(crate) base_backoff: Duration,
    pub(crate) multiplier: u32,
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
    pub(crate) fn backoff(&self, attempt: u32) -> Duration {
        crate::provider::retry::timing::jittered(self.base_backoff, self.multiplier, attempt)
    }

    pub(crate) fn delay(&self, attempt: u32, stop: &StreamStop) -> Duration {
        let suggested = match stop {
            StreamStop::Fault { message, .. } => {
                crate::provider::retry::timing::from_message(message)
            }
            _ => None,
        };
        suggested.unwrap_or_else(|| self.backoff(attempt))
    }
}
