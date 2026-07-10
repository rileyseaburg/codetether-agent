//! Converts wall-clock OAuth expiry timestamps into monotonic cache deadlines.

use std::time::{Duration, Instant};

const MAX_CACHE_AGE: Duration = Duration::from_secs(60 * 60);

pub(super) fn cache_deadline(expires_at: u64, now_epoch: u64, now: Instant) -> Instant {
    let remaining = Duration::from_secs(expires_at.saturating_sub(now_epoch));
    now.checked_add(remaining.min(MAX_CACHE_AGE)).unwrap_or(now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expired_timestamp_uses_current_instant() {
        let now = Instant::now();
        assert_eq!(cache_deadline(99, 100, now), now);
    }

    #[test]
    fn unbounded_timestamp_is_capped_without_overflow() {
        let now = Instant::now();
        assert_eq!(cache_deadline(u64::MAX, 100, now), now + MAX_CACHE_AGE);
    }
}
