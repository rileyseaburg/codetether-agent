//! Load a bounded materialized view without creating absent storage.
use anyhow::Result;
use rusqlite::{OptionalExtension, params};
use serde_json::Value;

/// Load a bounded materialized view without creating absent storage.
/// # Arguments
/// `path` is a session locator; `id` and `name` select the view.
/// # Returns
/// A view with its latest 128 documents in ascending order, or `None`.
/// # Errors
/// SQLite, JSON decoding or the blocking worker fails.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = codetether_session_store::projection::read(path, "session-123", "recall").await?;
/// # Ok(()) }
/// ```
pub async fn read(path: &std::path::Path, id: &str, name: &str) -> Result<Option<Value>> {
    let path = path.to_path_buf();
    let id = id.to_owned();
    let name = name.to_owned();
    tokio::task::spawn_blocking(move || {
        if !path.parent().unwrap().join("sessions.sqlite3").exists() { return Ok(None); }
        let db = crate::connection::open(&path)?;
        let Some(header) = db.query_row("SELECT header FROM views WHERE session_id=?1 AND name=?2", params![id,name], |r| r.get::<_,String>(0)).optional()? else { return Ok(None); };
        let mut view: Value = serde_json::from_str(&header)?;
        let mut query = db.prepare("SELECT body FROM projections WHERE session_id=?1 AND name=?2 ORDER BY seq DESC LIMIT 128")?;
        let mut docs = query.query_map(params![id,name], |r| r.get::<_,String>(0))?
            .map(|r| Ok(serde_json::from_str::<Value>(&r?)?)).collect::<Result<Vec<_>>>()?;
        docs.reverse(); view["documents"] = Value::Array(docs);
        Ok(Some(view))
    }).await?
}
