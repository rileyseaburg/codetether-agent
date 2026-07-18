//! Structured diagnostics for private Codex transport retries.

pub(super) fn retrying(attempt: u32, reason: &str) {
    tracing::warn!(attempt, reason, "Codex transport retrying privately");
}
