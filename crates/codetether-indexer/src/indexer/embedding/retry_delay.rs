//! retry delay for codebase indexing.

use super::*;

pub(in crate::indexer) fn retry_delay(attempt: u32, initial: Duration, max: Duration) -> Duration {
    let multiplier = 2u128.saturating_pow(attempt);
    let initial_ms = initial.as_millis();
    let max_ms = max.as_millis().max(initial_ms);
    let delay_ms = initial_ms.saturating_mul(multiplier).min(max_ms);
    Duration::from_millis(delay_ms as u64)
}
