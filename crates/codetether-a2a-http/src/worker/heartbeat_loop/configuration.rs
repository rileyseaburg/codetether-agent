//! Read persistent-worker heartbeat settings when the loop begins.
use super::*;
pub(super) fn persistent_settings() -> (bool, u64) {
    (
        persistent_worker_enabled(),
        persistent_worker_lease_seconds(),
    )
}
