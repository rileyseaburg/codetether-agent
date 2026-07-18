//! Delay selection for private Codex transport retries.

use std::time::Duration;

pub(super) fn before(attempt: u32, reason: &str) -> Duration {
    crate::provider::retry::timing::from_message(reason).unwrap_or_else(|| {
        crate::provider::retry::timing::jittered(Duration::from_millis(200), 2, attempt)
    })
}
