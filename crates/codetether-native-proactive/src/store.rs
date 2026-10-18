//! Validated paths and I/O for prepared summary-index sidecars.
pub use codetether_prepared_context::Prepared;
use std::path::PathBuf;
/// Read a prepared index after validating its owning session identity.
/// # Arguments
/// * session_id - Durable session identity.
/// # Returns
/// The readable prepared sidecar, or None when validation or reading fails.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let prepared = codetether_native_proactive::store::read_id("session-id").await;
/// # }
/// ```
pub async fn read_id(session_id: &str) -> Option<Prepared> {
    let path = path(session_id).ok()?;
    codetether_prepared_context::read(&path).await
}
/// Remove a sidecar when its session identity resolves successfully.
/// # Arguments
/// * session_id - Durable session identity.
/// # Returns
/// Completes after best-effort removal.
/// # Examples
/// ```no_run
/// # async fn example() {
/// codetether_native_proactive::store::remove("session-id").await;
/// # }
/// ```
pub async fn remove(session_id: &str) {
    if let Ok(path) = path(session_id) {
        codetether_prepared_context::remove(&path).await;
    }
}
/// Persist the prepared index using native session identity validation.
/// # Arguments
/// * session_id - Durable session identity.
/// * value - Prepared index and transcript fingerprint.
/// # Returns
/// Unit after successful persistence.
/// # Errors
/// Propagates path validation and shared prepared-store write failures.
/// # Examples
/// ```no_run
/// # async fn example(prepared: &codetether_native_proactive::store::Prepared) -> anyhow::Result<()> {
/// codetether_native_proactive::store::write("session-id", prepared).await?;
/// # Ok(()) }
/// ```
pub async fn write(session_id: &str, value: &Prepared) -> anyhow::Result<()> {
    let path = path(session_id)?;
    codetether_prepared_context::write(&path, value).await
}
fn path(session_id: &str) -> anyhow::Result<PathBuf> {
    codetether_session_store::lifecycle::session_path(session_id)?;
    Ok(codetether_session_store::lifecycle::sessions_dir()?
        .join("rlm")
        .join(format!("{session_id}.json")))
}
