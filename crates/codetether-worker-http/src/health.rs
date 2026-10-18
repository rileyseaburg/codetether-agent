//! Liveness response.

/// Health check - always returns OK if the server is running
pub(super) async fn health() -> &'static str {
    "ok"
}
