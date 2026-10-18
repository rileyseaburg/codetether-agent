//! Explicit external imports replace a session only against its current revision.
use super::Legacy;
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::path::Path;
/// Persist a newer external snapshot against the current indexed revision.
/// # Arguments
/// `session` is the incoming decoded snapshot; `path` selects its destination.
/// # Returns
/// Whether a write occurred. An existing snapshot at least as recent as the
/// incoming snapshot is preserved without writing.
/// # Errors
/// Propagates load/import, encoding, revision, locator and database failures.
/// # Panics
/// Requires a Tokio runtime; poisoned prior checkpoint state may panic.
/// # Examples
/// ```rust,no_run
/// # async fn example<S>(session: S, path: &std::path::Path) -> anyhow::Result<()>
/// # where S: codetether_session_store::legacy::Legacy + Send + Sync + 'static,
/// # S::Tool: serde::de::DeserializeOwned {
/// let written = codetether_session_store::legacy::persist(session, path).await?;
/// # Ok(()) }
/// ```
pub async fn persist<S: Legacy + Send + Sync + 'static>(mut session: S, path: &Path) -> Result<bool>
where
    S::Tool: DeserializeOwned,
{
    if path.exists() {
        let previous = crate::load::<S>(path, 0, super::ensure::<S>).await?.session;
        if previous.is_at_least_as_new_as(&session) {
            return Ok(false);
        }
        let mut checkpoint = previous.state().0.lock().unwrap().clone();
        checkpoint.message_start = 0;
        checkpoint.tool_start = 0;
        session.replace_state(crate::State(std::sync::Mutex::new(checkpoint)));
    }
    crate::save(&session, path).await
}
