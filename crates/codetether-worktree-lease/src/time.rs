//! Lease expiry policy used only to recover from dead agent processes.

pub(super) const TTL_MILLIS: i64 = 90_000;

pub(super) fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub(super) fn expiry() -> i64 {
    now_ms() + TTL_MILLIS
}

pub(super) fn remaining(expires_at_ms: i64) -> u64 {
    expires_at_ms.saturating_sub(now_ms()) as u64
}
