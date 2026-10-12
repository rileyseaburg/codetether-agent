//! Buffered response classification without inspecting successful content.

use super::classify::{is_permanent_message, is_retryable_message, is_retryable_status};
use super::{MAX_ATTEMPTS, delay::backoff_delay};

pub(super) async fn handle(
    text: String,
    status: reqwest::StatusCode,
    attempt: u32,
) -> anyhow::Result<Option<(String, reqwest::StatusCode)>> {
    if status.is_success() {
        return Ok(Some((text, status)));
    }
    if !is_retryable_status(status) && !is_retryable_message(&text) {
        anyhow::bail!("API error: {status} {text}");
    }
    if is_permanent_message(&text) {
        anyhow::bail!("Non-retryable provider error: {status} {text}");
    }
    if attempt >= MAX_ATTEMPTS {
        anyhow::bail!("API error after {attempt} attempts: {status} {text}");
    }
    let delay = backoff_delay(attempt);
    tracing::warn!(attempt, %status, delay_secs = delay.as_secs(), "Transient API error, retrying");
    tokio::time::sleep(delay).await;
    Ok(None)
}
