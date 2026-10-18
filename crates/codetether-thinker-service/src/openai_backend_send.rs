//! Application diagnostic for retryable transport failures.
pub(super) fn log_transient(e: &reqwest::Error) {
    tracing::warn!(target: "codetether_agent::cognition::thinker::openai_backend_send", error = %e, "thinker HTTP request failed (transient)");
}
