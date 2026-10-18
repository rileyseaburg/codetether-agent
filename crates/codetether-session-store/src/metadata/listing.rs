//! Metadata-only listing never deserializes transcript records.
use anyhow::Result;
use std::path::{Path, PathBuf};
/// Query ordered session metadata and project each header into a caller type.
/// # Arguments
/// `root` contains the session database; `workspace` optionally filters it;
/// `decode` maps a serialized header and durable message count into a result.
/// # Returns
/// Caller values in descending update order, or an empty list if no database exists.
/// # Errors
/// Propagates database, row-decoding and caller-projection failures.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(root: &std::path::Path) -> anyhow::Result<()> {
/// let rows = codetether_session_store::metadata::listing::list(
///     root, None, |header, count| Ok((header.to_owned(), count)),
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn list<T: Send + 'static>(
    root: &Path,
    workspace: Option<PathBuf>,
    decode: fn(&str, usize) -> Result<T>,
) -> Result<Vec<T>> {
    let root = root.to_path_buf();
    tokio::task::spawn_blocking(move || read(&root, workspace.as_deref(), decode)).await?
}
/// Query ordered session metadata and project each header into a caller type.
/// # Arguments
/// `root` contains the session database; `workspace` optionally filters it;
/// `decode` maps a serialized header and durable message count into a result.
/// # Returns
/// Caller values in descending update order, or an empty list if no database exists.
/// # Errors
/// Propagates database, row-decoding and caller-projection failures.
/// # Examples
/// ```rust,no_run
/// # fn example(root: &std::path::Path) -> anyhow::Result<()> {
/// let rows = codetether_session_store::metadata::listing::read(
///     root, None, |header, count| Ok((header.to_owned(), count)),
/// )?;
/// # Ok(()) }
/// ```
pub fn read<T>(
    root: &Path,
    workspace: Option<&Path>,
    decode: impl Fn(&str, usize) -> Result<T>,
) -> Result<Vec<T>> {
    if !root.join("sessions.sqlite3").exists() {
        return Ok(Vec::new());
    }
    let db = crate::connection::open(&root.join("locator.json"))?;
    let filter = if workspace.is_some() {
        " WHERE json_extract(header,'$.metadata.directory')=?1"
    } else {
        ""
    };
    let sql = format!(
        "SELECT header,message_count FROM sessions{filter} ORDER BY json_extract(header,'$.updated_at') DESC"
    );
    let mut query = db.prepare(&sql)?;
    let workspace = workspace.map(|path| path.to_string_lossy().into_owned());
    let rows = query.query_map(rusqlite::params_from_iter(workspace.iter()), |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, usize>(1)?))
    })?;
    let mut summaries = Vec::new();
    for row in rows {
        let (header, count) = row?;
        summaries.push(decode(&header, count)?);
    }
    Ok(summaries)
}
