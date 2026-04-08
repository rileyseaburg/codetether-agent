use super::classify::{backoff_delay, is_retryable_message, is_retryable_status};

/// Send an HTTP request with automatic retry on transient errors.
///
/// Wraps any async request factory in an infinite retry loop with
/// exponential backoff. Used by provider `complete` methods to survive
/// 429/502/503 responses, Z.AI "temporarily overloaded" errors, and
/// transient network failures before parsing the JSON body.
///
/// # Arguments
///
/// * `f` — A closure that sends the HTTP request, reads the full response
///   body, and returns `(body_text, status_code)`. Must be `FnMut` because
///   it is invoked on every retry attempt.
///
/// # Returns
///
/// The first successful `(body_text, status_code)` pair.
/// Non-retryable errors (4xx other than 429) bail immediately.
///
/// # Example
///
/// ```rust,no_run
/// let (text, _status) = send_with_retry(|| async {
///     let resp = client
///         .post("https://api.z.ai/v4/chat/completions")
///         .bearer_auth(&token)
///         .json(&body)
///         .send()
///         .await?;
///     let status = resp.status();
///     let text = resp.text().await?;
///     Ok((text, status))
/// })
/// .await?;
/// ```
pub async fn send_with_retry<F, Fut>(
    mut f: F,
) -> anyhow::Result<(String, reqwest::StatusCode)>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<
        Output = anyhow::Result<(String, reqwest::StatusCode)>,
    >,
{
    let mut attempt = 0u32;
    loop {
        attempt += 1;
        match f().await {
            Ok((text, status))
                if status.is_success()
                    && !is_retryable_message(&text) =>
            {
                return Ok((text, status));
            }
            Ok((text, status))
                if is_retryable_status(status)
                    || is_retryable_message(&text) =>
            {
                let delay = backoff_delay(attempt);
                tracing::warn!(
                    attempt, %status,
                    delay_secs = delay.as_secs(),
                    "Transient API error, retrying"
                );
                tokio::time::sleep(delay).await;
            }
            Ok((text, status)) => {
                anyhow::bail!("API error: {status} {text}");
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
