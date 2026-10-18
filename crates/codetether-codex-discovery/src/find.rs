//! Resolve a session ID to the first matching archive path.
use crate::{meta::read_session_meta, paths::is_jsonl};
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Find the first archive whose metadata matches a session ID.
/// # Arguments
/// * `codex_home` — Archive home containing the sessions directory.
/// * `id` — Session identifier to find.
/// # Returns
/// The matching archive path, if found.
/// # Errors
/// Propagates errors reading candidate metadata.
/// # Examples
/// ```no_run
/// let result = codetether_codex_discovery::find_codex_session_path_by_id(std::path::Path::new(".codex"), "id");
/// assert!(result.is_ok());
/// ```
pub fn find_codex_session_path_by_id(codex_home: &Path, id: &str) -> Result<Option<PathBuf>> {
    let sessions_root = codex_home.join("sessions");
    if !sessions_root.exists() {
        return Ok(None);
    }
    for entry in WalkDir::new(&sessions_root)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();
        if !is_jsonl(path) {
            continue;
        }
        if let Some(meta) = read_session_meta(path)?
            && meta.id == id
        {
            return Ok(Some(path.to_path_buf()));
        }
    }
    Ok(None)
}
