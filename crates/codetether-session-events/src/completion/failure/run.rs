use super::Host;

/// Apply a failed completion and schedule recovery in priority order.
///
/// # Arguments
/// * `host` - Native session ownership, recovery and presentation bindings.
/// * `session` - Owned session returned by the runtime.
/// * `error` - Error text used for scheduling and then moved into the transcript.
/// # Returns
/// Completes after recovery scheduling and visible failure updates.
/// # Examples
/// ```text
/// failure::apply(&mut native_host, session, error).await;
/// ```
pub async fn apply<H: Host>(host: &mut H, session: H::Session, error: String) {
    host.restore(session);
    host.stopped().await;
    host.complete_timing();
    host.reconnect(&error);
    host.smart_switch(&error);
    host.error(error);
    *host.status() = "Request failed".to_string();
    host.scroll();
}
