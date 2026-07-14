//! Bounded private retries for Codex streaming transports.

use anyhow::Result;
use futures::StreamExt;
use std::{future::Future, time::Duration};

use super::{Chunks, StreamChunk, attempt, attempt::Outcome, classify};

const MAX_HTTP_ATTEMPTS: u32 = 3;

/// Retry pre-content transient failures privately with bounded backoff.
pub(in crate::provider::openai_codex) fn with_http_retry<F, Fut>(
    primary: Option<Chunks>,
    mut retry: F,
) -> Chunks
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = Result<Chunks>> + Send + 'static,
{
    Box::pin(async_stream::stream! {
        if let Some(primary) = primary {
            match attempt::inspect(primary).await {
                Outcome::Ready(first, mut remaining) => {
                    yield first;
                    while let Some(chunk) = remaining.next().await { yield chunk; }
                    return;
                }
                Outcome::Retry(reason) => log_retry(0, &reason),
            }
        }
        for number in 1..=MAX_HTTP_ATTEMPTS {
            let outcome = match retry().await {
                Ok(stream) => attempt::inspect(stream).await,
                Err(error) if classify::is_retryable(&format!("{error:#}")) => Outcome::Retry(format!("{error:#}")),
                Err(error) => Outcome::Ready(StreamChunk::Error(format!("{error:#}")), Box::pin(futures::stream::empty())),
            };
            match outcome {
                Outcome::Ready(first, mut remaining) => {
                    yield first;
                    while let Some(chunk) = remaining.next().await { yield chunk; }
                    return;
                }
                Outcome::Retry(reason) if number < MAX_HTTP_ATTEMPTS => {
                    log_retry(number, &reason);
                    tokio::time::sleep(Duration::from_secs(1 << (number - 1))).await;
                }
                Outcome::Retry(reason) => {
                    yield StreamChunk::Error(format!("temporary Codex transport failure after {number} attempts: {reason}"));
                }
            }
        }
    })
}

fn log_retry(attempt: u32, reason: &str) {
    tracing::warn!(attempt, reason, "Codex transport retrying privately");
}
