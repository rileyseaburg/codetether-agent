//! Explicit-session scanning with lazy resume-window lookup.
use super::Host;
use codetether_session_store::TailLoad;
/// Load an explicitly requested session without allocating a fresh one.
/// # Arguments
/// Receives an optional session identifier and a lazily evaluated history-window policy.
/// # Returns
/// The bounded session window when explicitly requested.
/// # Errors
/// Returns an expected-absence error for no identifier, or a session loading error.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_tui_startup::resolve::Host>() {
/// let _ = codetether_tui_startup::resolve::scan::<H>(None, || 0).await;
/// # }
/// ```
pub async fn scan<H: Host>(
    session_id: Option<&str>,
    window: impl FnOnce() -> usize,
) -> anyhow::Result<TailLoad<H::Session>> {
    let Some(session_id) = session_id else {
        return Err(anyhow::anyhow!("requested fresh session"));
    };
    H::load_tail(session_id, window()).await
}
