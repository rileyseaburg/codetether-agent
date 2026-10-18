//! Read, write, and remove individual session recall sidecars.

use anyhow::Result;

use codetether_recall_index::IndexedSession;

/// Read a materialized projection before trying its legacy sidecar.
/// # Arguments
/// Receives a session ID and native storage host.
/// # Returns
/// A decoded projection, a schema-compatible sidecar, or None.
/// # Examples
/// ```no_run
/// # async fn example(h: &impl codetether_recall_store::Host) -> anyhow::Result<()> {
/// let index = codetether_recall_store::read("session-id", h).await;
/// # Ok(())
/// # }
/// ```
pub async fn read<H: crate::Host>(session_id: &str, host: &H) -> Option<IndexedSession> {
    if let Ok(Some(view)) = host.projection(session_id).await {
        return serde_json::from_value(view).ok();
    }
    let path = super::paths::session(session_id, host).ok()?;
    let indexed: IndexedSession = super::atomic::read(&path).await?;
    indexed.is_current_schema().then_some(indexed)
}

/// Write one legacy sidecar using atomic JSON replacement.
/// # Arguments
/// Receives a projection and native storage host.
/// # Returns
/// Completes after replacing the sidecar.
/// # Errors
/// Propagates path, serialization and filesystem errors.
/// # Examples
/// ```no_run
/// # async fn example(h: &impl codetether_recall_store::Host, index: &codetether_recall_index::IndexedSession) -> anyhow::Result<()> {
/// codetether_recall_store::write(index, h).await
/// # }
/// ```
pub async fn write<H: crate::Host>(indexed: &IndexedSession, host: &H) -> Result<()> {
    super::atomic::write(&super::paths::session(&indexed.session_id, host)?, indexed).await
}

/// Remove only a legacy sidecar, tolerating absence.
/// # Arguments
/// Receives a session ID and native storage host.
/// # Returns
/// Completes without changing workspace catalog entries.
/// # Errors
/// Propagates invalid IDs and filesystem errors other than NotFound.
/// # Examples
/// ```no_run
/// # async fn example(h: &impl codetether_recall_store::Host) -> anyhow::Result<()> {
/// codetether_recall_store::remove_sidecar("session-id", h).await?;
/// # Ok(())
/// # }
/// ```
pub async fn remove<H: crate::Host>(session_id: &str, host: &H) -> Result<()> {
    let path = super::paths::session(session_id, host)?;
    match tokio::fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}
