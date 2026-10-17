//! Preserve the rate-limit delay formula, logging and asynchronous pause.
use tracing::warn;
pub(super) async fn pause(
    attempt: usize,
    max_rate_limit_retries: usize,
    rate_limit_base_delay_ms: u64,
    story_id: &str,
) {
    let delay_ms = rate_limit_base_delay_ms * 2u64.saturating_pow(attempt as u32);
    warn!(
        story_id = %story_id,
        attempt = attempt + 1,
        max_retries = max_rate_limit_retries,
        delay_ms = delay_ms,
        "Rate limited, backing off before retry"
    );
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
}
