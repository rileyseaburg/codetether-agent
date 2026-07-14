//! Bounded private retries for Codex streaming transports.

use anyhow::Result;
use std::{future::Future, time::Duration};

use super::{Chunks, StreamChunk, attempt, attempt::Outcome, classify};

const MAX_HTTP_ATTEMPTS: u32 = 3;

pub(in crate::provider::openai_codex) fn with_http_retry<F, Fut>(
    primary: Chunks,
    mut retry: F,
) -> Chunks
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = Result<Chunks>> + Send + 'static,
{
    Box::pin(async_stream::stream! {
        match attempt::collect(primary).await {
            Outcome::Complete(chunks) | Outcome::Permanent(chunks) => {
                for chunk in chunks { yield chunk; }
                return;
            }
            Outcome::Retry(reason) => log_retry(0, &reason),
        }
        for number in 1..=MAX_HTTP_ATTEMPTS {
            let outcome = match retry().await {
                Ok(stream) => attempt::collect(stream).await,
                Err(error) if classify::is_retryable(&format!("{error:#}")) => Outcome::Retry(format!("{error:#}")),
                Err(error) => Outcome::Permanent(vec![StreamChunk::Error(format!("{error:#}"))]),
            };
            match outcome {
                Outcome::Complete(chunks) | Outcome::Permanent(chunks) => {
                    for chunk in chunks { yield chunk; }
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
