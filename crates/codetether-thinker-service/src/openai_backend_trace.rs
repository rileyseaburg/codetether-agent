//! Application retry and success diagnostics.
use crate::ThinkerOutput;
use std::time::Instant;

pub(super) fn log_retry(attempt: u32) {
    tracing::debug!(target: "codetether_agent::cognition::thinker::openai_backend_trace", attempt, "retrying thinker HTTP request");
}

/// Record a completed request at debug level.
pub(super) fn log_success(output: &ThinkerOutput, started_at: Instant, attempt: u32) {
    tracing::debug!(target: "codetether_agent::cognition::thinker::openai_backend_trace",
        model = %output.model,
        latency_ms = started_at.elapsed().as_millis(),
        prompt_tokens = ?output.prompt_tokens,
        completion_tokens = ?output.completion_tokens,
        attempt,
        "openai-compat thinker generated thought"
    );
}
