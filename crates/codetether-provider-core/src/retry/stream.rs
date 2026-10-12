//! Streaming request retry orchestration before body consumption.

use super::{network, stream_body};

/// Send a streaming request with at most five attempts on transient errors.
///
/// # Arguments
///
/// * `f` — Factory producing a fresh response for each attempt.
///
/// # Returns
///
/// The first successful response, without consuming its streaming body.
///
/// # Errors
///
/// Returns non-retryable failures immediately and transient failures after five
/// attempts. Retryable HTTP statuses with permanent billing errors stop early.
///
/// # Examples
///
/// ```rust
/// # tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap().block_on(async {
/// use codetether_provider_core::retry::send_response_with_retry;
/// let response = send_response_with_retry(|| async {
///     Ok(http::Response::builder().status(200).body("event".to_owned()).unwrap().into())
/// }).await.unwrap();
/// assert_eq!(response.text().await.unwrap(), "event");
/// # });
/// ```
pub async fn send_response_with_retry<F, Fut>(mut f: F) -> anyhow::Result<reqwest::Response>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<reqwest::Response>>,
{
    let mut attempt = 0;
    loop {
        attempt += 1;
        match f().await {
            Ok(response) => {
                if let Some(response) = stream_body::handle(response, attempt).await? {
                    return Ok(response);
                }
            }
            Err(error) => network::streaming(error, attempt).await?,
        }
    }
}
