//! Dirty-write gating and post-commit notifications.
use super::Host;
use crate::Snapshot;
use anyhow::Result;
/// Save the changed session and publish notifications only after a write.
///
/// # Arguments
/// * `session` — Snapshot and checkpoint to persist.
/// # Returns
/// Success after a clean no-op or a committed write and notifications.
/// # Errors
/// Path, encoding, revision, write or checkpoint failures.
/// # Panics
/// Panics if checkpoint synchronization is poisoned or a required Tokio runtime is absent.
/// # Examples
/// ```no_run
/// # async fn run<H: codetether_session_store::lifecycle::Host>(session: &H::Session) -> anyhow::Result<()> {
/// let _result = codetether_session_store::lifecycle::save::<H>(session).await?;
/// # Ok(())
/// # }
/// ```
pub async fn save<H: Host>(session: &H::Session) -> Result<()> {
    let path = session
        .state()
        .path_for(session.id())
        .map(Ok)
        .unwrap_or_else(|| super::session_path(session.id()))?;
    if crate::save(session, &path).await? {
        codetether_session_location::record(session.id(), &path);
        H::schedule_recall(session);
        H::notify_saved(session);
        H::schedule_archive(session);
    }
    Ok(())
}
