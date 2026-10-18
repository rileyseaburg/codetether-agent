//! Application diagnostic for retryable HTTP statuses.
pub(super) fn log_transient(status: reqwest::StatusCode) {
    tracing::warn!(target: "codetether_agent::cognition::thinker::openai_backend_status", %status, "thinker received transient HTTP error");
}
