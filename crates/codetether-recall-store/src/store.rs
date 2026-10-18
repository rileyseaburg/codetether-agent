//! Transactional orchestration for recall sidecars and catalogs.

use anyhow::Result;
use codetether_recall_index::IndexedSession;

/// Persist a current projection and include its session in the workspace catalog.
/// # Arguments
/// Receives a candidate projection and the native storage host.
/// # Returns
/// Completes after both writes, or skips deleted, missing or stale sessions.
/// # Errors
/// Propagates path resolution, serialization and filesystem errors.
/// # Examples
/// ```no_run
/// # async fn example(h: &impl codetether_recall_store::Host) -> anyhow::Result<()> {
/// let _ = h; // Supply a projection built by codetether_recall_index::project.
/// # async fn save(h: &impl codetether_recall_store::Host, index: codetether_recall_index::IndexedSession) -> anyhow::Result<()> {
/// codetether_recall_store::upsert(index, h).await
/// # }
/// # Ok(())
/// # }
/// ```
pub async fn upsert<H: crate::Host>(indexed: IndexedSession, host: &H) -> Result<()> {
    let _guard = super::store_lock::acquire().await;
    if super::tombstone::contains(&indexed.session_id) {
        return Ok(());
    }
    if !host.session_path(&indexed.session_id)?.exists() {
        return Ok(());
    }
    if super::store_freshness::stale(&indexed, host).await {
        return Ok(());
    }
    super::session_io::write(&indexed, host).await?;
    let mut catalog = super::catalog_io::read(&indexed.workspace, host).await;
    catalog.insert(&indexed.session_id);
    super::catalog_io::write(&catalog, host).await?;
    Ok(())
}

/// Remove the sidecar and its workspace catalog entry under the mutation lock.
/// # Arguments
/// Receives a session ID and native storage host.
/// # Returns
/// Completes after removal; an absent sidecar is accepted.
/// # Errors
/// Propagates invalid IDs and filesystem errors.
/// # Examples
/// ```no_run
/// # async fn example(h: &impl codetether_recall_store::Host) -> anyhow::Result<()> {
/// codetether_recall_store::remove("session-id", h).await?;
/// # Ok(())
/// # }
/// ```
pub async fn remove<H: crate::Host>(session_id: &str, host: &H) -> Result<()> {
    let _guard = super::store_lock::acquire().await;
    let Some(indexed) = super::session_io::read(session_id, host).await else {
        super::session_io::remove(session_id, host).await?;
        return Ok(());
    };
    super::session_io::remove(session_id, host).await?;
    let mut catalog = super::catalog_io::read(&indexed.workspace, host).await;
    catalog.remove(session_id);
    super::catalog_io::write(&catalog, host).await?;
    Ok(())
}
