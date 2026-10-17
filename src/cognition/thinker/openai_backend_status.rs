//! Application diagnostic for retryable HTTP statuses.
pub(super) fn log_transient(status: reqwest::StatusCode) {
    tracing::warn!(%status, "thinker received transient HTTP error");
}
