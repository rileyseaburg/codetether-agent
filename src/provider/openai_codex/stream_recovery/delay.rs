//! Delay selection for private Codex transport retries.

use std::time::Duration;

pub(super) fn before(attempt: u32) -> Duration {
    crate::provider::retry::timing::jittered(Duration::from_millis(200), 2, attempt)
}
