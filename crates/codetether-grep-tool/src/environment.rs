//! environment for bounded grep searches.
use super::*;
pub(super) fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}
pub(super) fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.parse().ok()
}
pub(super) fn remaining_duration(deadline: Instant) -> Option<Duration> {
    deadline.checked_duration_since(Instant::now())
}
