//! Resolve a session location before reading its materialized consumer view.
/// Read a materialized view by durable session identity.
/// # Arguments
/// `id` identifies the session and `name` selects its consumer projection.
/// # Returns
/// The existing view, or `None` when no materialized view exists.
/// # Errors
/// Propagates location resolution, SQLite, JSON and blocking-task failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _view = codetether_session_store::projection::read_session("session", "recall").await?;
/// # Ok(()) }
/// ```
pub async fn read_session(id: &str, name: &str) -> anyhow::Result<Option<serde_json::Value>> {
    let path = crate::lifecycle::session_path(id)?;
    super::read(&path, id, name).await
}
