//! Streaming response classification; successful bodies remain unread.

use super::classify::{is_permanent_message, is_retryable_message, is_retryable_status};
use super::{MAX_ATTEMPTS, delay::backoff_delay};

pub(super) async fn handle(
    response: reqwest::Response,
    attempt: u32,
) -> anyhow::Result<Option<reqwest::Response>> {
    let status = response.status();
    if status.is_success() {
        return Ok(Some(response));
    }
    let retryable_status = is_retryable_status(status);
    let body = response.text().await.unwrap_or_default();
    if retryable_status && is_permanent_message(&body) {
        anyhow::bail!("Non-retryable provider error: {status} {body}");
    }
    if !retryable_status && !is_retryable_message(&body) {
        anyhow::bail!("Streaming error: {status} {body}");
    }
    if attempt >= MAX_ATTEMPTS {
        anyhow::bail!("Streaming error after {attempt} attempts: {status} {body}");
    }
    let delay = backoff_delay(attempt);
    let reason = if retryable_status {
        "Transient streaming error, retrying"
    } else {
        "Transient streaming error (body), retrying"
    };
    tracing::warn!(attempt, %status, delay_secs = delay.as_secs(), "{reason}");
    tokio::time::sleep(delay).await;
    Ok(None)
}
