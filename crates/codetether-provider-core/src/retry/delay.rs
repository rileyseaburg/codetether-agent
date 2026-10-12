//! Deterministic retry delays and shared network warning emission.

use std::time::Duration;

pub(super) fn backoff_delay(attempt: u32) -> Duration {
    Duration::from_secs(2u64.saturating_pow(attempt).min(30))
}

pub(super) async fn network(attempt: u32, error: &anyhow::Error) {
    let delay = backoff_delay(attempt);
    tracing::warn!(attempt, %error, delay_secs = delay.as_secs(), "Transient network error, retrying");
    tokio::time::sleep(delay).await;
}
