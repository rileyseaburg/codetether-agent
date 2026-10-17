//! Application diagnostic for retryable transport failures.
pub(super) fn log_transient(e: &reqwest::Error) {
    tracing::warn!(error = %e, "thinker HTTP request failed (transient)");
}
