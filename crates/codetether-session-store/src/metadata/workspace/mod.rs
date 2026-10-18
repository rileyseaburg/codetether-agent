//! Resolve persisted session workspace metadata with legacy path fallback.
mod projection;
use anyhow::{Context, Result};
use projection::WorkspaceProjection;
use std::path::{Path, PathBuf};

/// Returns the workspace directory recorded for session `id`.
///
/// When the snapshot records no workspace, the directory is derived from the
/// snapshot's own location so sessions written before `metadata.directory`
/// existed remain resumable.
///
/// # Errors
///
/// Returns an error if the snapshot is missing, the JSON
/// cannot be parsed, or no directory can be determined.
/// # Arguments
/// `path` is the caller-resolved session locator or legacy snapshot; `id`
/// identifies the session in diagnostics. The caller owns identity validation.
/// # Returns
/// An existing recorded directory or the snapshot layout's enclosing workspace.
/// # Examples
/// ```rust,no_run
/// # async fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let workspace = codetether_session_store::metadata::workspace::recorded_workspace(path, "session-123").await?;
/// assert!(workspace.is_dir());
/// # Ok(()) }
/// ```
pub async fn recorded_workspace(path: &Path, id: &str) -> Result<PathBuf> {
    let body = match crate::header_read::body(path).await? {
        Some(header) => header,
        None => tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("read session {id}"))?,
    };
    let projection: WorkspaceProjection =
        serde_json::from_str(&body).with_context(|| format!("parse session {id}"))?;
    if let Some(directory) = projection.metadata.directory.filter(|item| item.is_dir()) {
        return Ok(directory);
    }
    enclosing_workspace(path).with_context(|| format!("resolve workspace for session {id}"))
}

/// Derives the workspace from the snapshot path, which is laid out as
/// `<workspace>/.codetether-agent/sessions/<id>.json`.
fn enclosing_workspace(session_file: &Path) -> Option<PathBuf> {
    session_file
        .ancestors()
        .nth(3)
        .filter(|path| path.is_dir())
        .map(PathBuf::from)
}
#[cfg(test)]
mod tests;
