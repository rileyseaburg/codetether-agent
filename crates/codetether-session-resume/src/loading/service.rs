//! Ordered native-tail and archive loading with preserved failure context.
use super::{Host, LoadedSession};
/// Load a bounded native session, falling back to archive import on failure.
/// # Arguments
/// * `id` - Requested session identity.
/// * `window` - Maximum retained native history window.
/// # Returns
/// Loaded session and window metadata; imported sessions have zero size/drop fields.
/// # Errors
/// Returns the native error with archive failure context if both loaders fail.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_session_resume::loading::Host>() {
/// let result = codetether_session_resume::loading::load::<H>("session", 100).await;
/// # }
/// ```
pub async fn load<H: Host>(id: &str, window: usize) -> anyhow::Result<LoadedSession<H::Session>> {
    match H::load_tail(id, window).await {
        Ok(load) => Ok(load),
        Err(native_error) => match H::load_or_import(id).await {
            Ok(session) => Ok(LoadedSession {
                session,
                dropped: 0,
                file_bytes: 0,
            }),
            Err(codex_error) => {
                Err(native_error.context(format!("Codex import also failed: {codex_error}")))
            }
        },
    }
}
