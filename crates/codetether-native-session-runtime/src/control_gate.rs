//! Bind shared execution control to application-owned durable goal checks.
pub(super) fn check(id: &str) -> codetether_session_control::GateFuture<'_> {
    Box::pin(super::boundary::ensure_running(id))
}
