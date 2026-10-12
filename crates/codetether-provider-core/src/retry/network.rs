//! Network-error handling for buffered and streaming request retries.

use super::classify::{is_permanent_message, is_retryable_message};
use super::{MAX_ATTEMPTS, delay};

pub(super) async fn buffered(error: anyhow::Error, attempt: u32) -> anyhow::Result<()> {
    if !is_retryable_message(&error.to_string()) || is_permanent_message(&error.to_string()) {
        return Err(error);
    }
    if attempt >= MAX_ATTEMPTS {
        return Err(error.context(format!("network error persisted after {attempt} attempts")));
    }
    delay::network(attempt, &error).await;
    Ok(())
}

pub(super) async fn streaming(error: anyhow::Error, attempt: u32) -> anyhow::Result<()> {
    if !is_retryable_message(&error.to_string()) || attempt >= MAX_ATTEMPTS {
        return Err(error);
    }
    delay::network(attempt, &error).await;
    Ok(())
}
