use super::classify::{backoff_delay, is_retryable_message, is_retryable_status};

/// Send a streaming request with automatic retry on transient HTTP errors.
///
/// Wraps any async request factory in an infinite retry loop with
/// exponential backoff. Used by provider `complete_stream` methods to
/// survive 429/502/503 responses and transient network failures before
/// the caller begins consuming the response body as an SSE stream.
///
/// # Arguments
///
/// * `f` — A closure that produces a fresh `reqwest::Response` on each
///   call. Must be `FnMut` because it is invoked on every retry attempt.
///
/// # Returns
///
/// The first successful (2xx) `Response`, ready for stream consumption.
/// Non-retryable errors (4xx other than 429) bail immediately.
///
/// # Example
///
/// ```rust,no_run
/// let resp = send_response_with_retry(|| async {
///     client
///         .post("https://api.z.ai/v4/chat/completions")
///         .bearer_auth(&token)
///         .json(&body)
///         .send()
///         .await
///         .context("Failed to send streaming request")
/// })
/// .await?;
/// let stream = resp.bytes_stream();
/// ```
pub async fn send_response_with_retry<F, Fut>(
    mut f: F,
) -> anyhow::Result<reqwest::Response>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<reqwest::Response>>,
{
    let mut attempt = 0u32;
    loop {
        attempt += 1;
        match f().await {
            Ok(resp) if resp.status().is_success() => {
                return Ok(resp);
            }
            Ok(resp) if is_retryable_status(resp.status()) => {
                let status = resp.status();
                // Drain body so the connection can be reused
                let _ = resp.bytes().await;
                let delay = backoff_delay(attempt);
                tracing::warn!(
                    attempt, %status,
                    delay_secs = delay.as_secs(),
                    "Transient streaming error, retrying"
                );
                tokio::time::sleep(delay).await;
            }
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if is_retryable_message(&text) {
                    let delay = backoff_delay(attempt);
                    tracing::warn!(
                        attempt, %status,
                        delay_secs = delay.as_secs(),
                        "Transient streaming error (body), retrying"
                    );
                    tokio::time::sleep(delay).await;
                    continue;
                }
                anyhow::bail!("Streaming error: {status} {text}");
            }
            Err(e) if is_retryable_message(&e.to_string()) => {
                let delay = backoff_delay(attempt);
                tracing::warn!(
                    attempt, error = %e,
                    delay_secs = delay.as_secs(),
                    "Transient network error, retrying"
                );
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}
