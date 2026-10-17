//! Application retry and success diagnostics.
use super::ThinkerOutput;
use std::time::Instant;

pub(super) fn log_retry(attempt: u32) {
    tracing::debug!(attempt, "retrying thinker HTTP request");
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
