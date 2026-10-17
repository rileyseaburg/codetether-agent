//! Structured reporting for interrupted WebSockets.

pub(super) fn record(session_id: &str, reason: &str) {
    tracing::warn!(session_id, reason, "Codex WebSocket stream interrupted");
}
