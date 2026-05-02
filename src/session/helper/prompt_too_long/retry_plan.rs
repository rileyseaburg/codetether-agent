//! Attempt-to-context-retention retry schedule.

const FORCE_KEEP_LAST_RETRIES: [usize; 3] = [6, 3, 1];

pub(super) fn keep_last_for_attempt(attempt: usize) -> Option<usize> {
    FORCE_KEEP_LAST_RETRIES
        .get(attempt.saturating_sub(1))
        .copied()
}
