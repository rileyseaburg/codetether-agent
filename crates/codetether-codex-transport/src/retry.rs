//! Bounded retries for opening Codex HTTP requests.

use anyhow::Result;
use futures::StreamExt;
use std::future::Future;

use super::{Chunks, StreamChunk, attempt, attempt::Outcome, classify, delay, log};

const MAX_REQUEST_RETRIES: u32 = 4;

/// Retry HTTP handshakes without consuming or restarting the response stream.
///
/// # Arguments
/// * `retry` - A fresh request-opening future for each attempt.
/// # Returns
/// The first opened stream, or a classified error chunk after exhaustion.
/// # Examples
/// ```no_run
/// use futures::StreamExt;
/// let stream = codetether_codex_transport::with_http_retry(|| async {
///     Ok(futures::stream::empty().boxed())
/// });
/// ```
pub fn with_http_retry<F, Fut>(mut retry: F) -> Chunks
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = Result<Chunks>> + Send + 'static,
{
    Box::pin(async_stream::stream! {
        for number in 0..=MAX_REQUEST_RETRIES {
            let outcome = attempt::open(retry().await).await;
            match outcome {
                Outcome::Ready(mut remaining) => {
                    while let Some(chunk) = remaining.next().await { yield chunk; }
                    return;
                }
                Outcome::Retry(reason) if number < MAX_REQUEST_RETRIES => {
                    log::retrying(number + 1, &reason);
                    tokio::time::sleep(delay::before(number)).await;
                }
                Outcome::Retry(reason) => {
                    let prefix = if classify::exhausted_request_retryable(&reason) {
                        "codex-retryable"
                    } else {
                        "codex-permanent"
                    };
                    yield StreamChunk::Error(format!("{prefix}: {reason}"));
                    return;
                }
                Outcome::Terminal(reason, retryable) => {
                    let prefix = if retryable { "codex-retryable" } else { "codex-permanent" };
                    yield StreamChunk::Error(format!("{prefix}: {reason}"));
                    return;
                }
            }
        }
    })
}
