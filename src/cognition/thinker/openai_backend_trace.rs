//! Retry pacing and success logging for the OpenAI-compatible backend.

use std::time::{Duration, Instant};

use super::ThinkerOutput;

/// Sleep before a retry attempt; the first attempt does not wait.
pub(super) async fn backoff(attempt: u32) {
    if attempt > 0 {
        tokio::time::sleep(Duration::from_millis(500 * u64::from(attempt))).await;
        tracing::debug!(attempt, "retrying thinker HTTP request");
    }
}

/// Record a completed request at debug level.
pub(super) fn log_success(output: &ThinkerOutput, started_at: Instant, attempt: u32) {
    tracing::debug!(
        model = %output.model,
        latency_ms = started_at.elapsed().as_millis(),
        prompt_tokens = ?output.prompt_tokens,
        completion_tokens = ?output.completion_tokens,
        attempt,
        "openai-compat thinker generated thought"
    );
}
