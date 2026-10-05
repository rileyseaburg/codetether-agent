//! Metadata-only listing never deserializes transcript records.
use crate::session::SessionSummary;
use anyhow::Result;
use std::path::{Path, PathBuf};
pub(crate) async fn list(root: &Path, workspace: Option<PathBuf>) -> Result<Vec<SessionSummary>> {
    let root = root.to_path_buf();
    tokio::task::spawn_blocking(move || read(&root, workspace.as_deref())).await?
}
fn read(root: &Path, workspace: Option<&Path>) -> Result<Vec<SessionSummary>> {
    if !root.join("sessions.sqlite3").exists() {
        return Ok(Vec::new());
    }
    let db = super::connection::open(&root.join("locator.json"))?;
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
        let session = super::header::decode(&header)?;
        let directory = session.metadata.directory;
        summaries.push(SessionSummary {
            id: session.id,
            title: session.title,
            created_at: session.created_at,
            updated_at: session.updated_at,
            message_count: count,
            agent: session.agent,
            directory,
        });
    }
    Ok(summaries)
}
