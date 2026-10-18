//! Delete authoritative rows before removing the compatibility locator.
use anyhow::Result;
use std::path::Path;
/// Remove authoritative rows in one transaction before locator cleanup.
/// # Arguments
/// `path` locates storage; `id` selects the session.
/// # Returns
/// Unit after deletion, or when the database is absent.
/// # Errors
/// SQLite operations or the blocking worker fails.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = codetether_session_store::delete::run(path, "session-123").await?;
/// # Ok(()) }
/// ```
pub async fn run(path: &Path, id: &str) -> Result<()> {
    let path = path.to_path_buf();
    let id = id.to_owned();
    tokio::task::spawn_blocking(move || {
        if !path.parent().unwrap().join("sessions.sqlite3").exists() {
            return Ok(());
        }
        let db = super::connection::open(&path)?;
        let tx = db.unchecked_transaction()?;
        for table in ["tool_calls", "commits", "views", "projections"] {
            tx.execute(&format!("DELETE FROM {table} WHERE session_id=?1"), [&id])?;
        }
        tx.execute("DELETE FROM sessions WHERE id=?1", [&id])?;
        tx.commit()?;
        anyhow::Ok(())
    })
    .await?
}
