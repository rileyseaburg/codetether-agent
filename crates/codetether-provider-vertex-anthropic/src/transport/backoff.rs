//! Existing exponential retry delay.
use crate::*;
pub(super) async fn wait(attempt: u32) {
    if attempt > 0 {
        let backoff = Duration::from_millis(1000 * 2u64.pow(attempt - 1));
        tracing::warn!(
            attempt,
            backoff_ms = backoff.as_millis() as u64,
            "Vertex Anthropic retrying after transient error"
        );
        tokio::time::sleep(backoff).await;
    }
}
