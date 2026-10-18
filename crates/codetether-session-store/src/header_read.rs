//! Read small metadata directly from SQLite without importing legacy projections.
use anyhow::Result;
use std::path::Path;
/// Read metadata directly when a session locator identifies database storage.
/// # Arguments
/// `path` selects the locator.
/// # Returns
/// The stored header, or `None` for a legacy non-locator file.
/// # Errors
/// Locator I/O, SQLite operations or the blocking worker fails.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = codetether_session_store::header_read::body(path).await?;
/// # Ok(()) }
/// ```
pub async fn body(path: &Path) -> Result<Option<String>> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        if !super::marker::is_marker(&path)? {
            return Ok(None);
        }
        let id = super::marker::id(&path)?;
        let db = super::connection::open(&path)?;
        Ok(Some(super::read::header(&db, &id)?.1))
    })
    .await?
}
