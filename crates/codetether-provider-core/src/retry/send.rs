//! Buffered request retry orchestration.

use super::{network, send_body};

/// Send an HTTP request with at most five attempts on transient errors.
///
/// # Arguments
///
/// * `f` — Factory returning the full response text and HTTP status each attempt.
///
/// # Returns
///
/// The first successful text/status pair; success content is never classified.
///
/// # Errors
///
/// Returns permanent provider failures immediately and transient errors after
/// five attempts, waiting 2, 4, 8 and 16 seconds between attempts.
///
/// # Examples
///
/// ```rust
/// # tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap().block_on(async {
/// use codetether_provider_core::retry::send_with_retry;
/// let (text, status) = send_with_retry(|| async {
///     Ok(("ok".into(), reqwest::StatusCode::OK))
/// }).await.unwrap();
/// assert_eq!(text, "ok");
/// assert!(status.is_success());
/// # });
/// ```
pub async fn send_with_retry<F, Fut>(mut f: F) -> anyhow::Result<(String, reqwest::StatusCode)>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<(String, reqwest::StatusCode)>>,
{
    let mut attempt = 0;
    loop {
        attempt += 1;
        match f().await {
            Ok((text, status)) => {
                if let Some(response) = send_body::handle(text, status, attempt).await? {
                    return Ok(response);
                }
            }
            Err(error) => network::buffered(error, attempt).await?,
        }
    }
}
