use super::{ActiveSession, Host};

/// Apply a successfully completed session to its current frontend slot.
///
/// # Arguments
/// * `host` - Session ownership, persistence and presentation bindings.
/// * `session` - Owned session returned by the runtime.
/// # Returns
/// Completes after updating the session list. Stale results are saved and
/// trigger a full list refresh without replacing the active slot.
/// # Examples
/// ```text
/// success::apply(&mut native_host, session).await;
/// ```
pub async fn apply<H: Host>(host: &mut H, session: H::Session) {
    if H::session_id(&session) != host.current_id() {
        host.warn_stale(&session);
        H::save(&session).await;
        host.refresh().await;
        return;
    }
    if host.processing() {
        host.stopped().await;
        host.clear_timing();
    }
    host.restore(session);
    let Some(mut active) = host.active() else {
        return;
    };
    active.attach_bus();
    active.reset_reconnect();
    active.set_session_id();
    active.save().await;
    active.upsert();
}
