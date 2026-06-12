use std::path::{Path, PathBuf};

use anyhow::Result;

use super::parse_summary::parse_summary;
use super::summary::SessionSummary;

/// Full-directory listing scan. Used as the rebuild/repair path when
/// `index.jsonl` is missing or stale. Runs entirely on the blocking
/// pool — it opens and header-parses every session file.
pub(super) async fn scan(
    sessions_dir: PathBuf,
    workspace: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    tokio::task::spawn_blocking(move || scan_sync(&sessions_dir, workspace.as_deref()))
        .await
        .map_err(|e| anyhow::anyhow!("listing scan task panicked: {e}"))?
}

fn scan_sync(sessions_dir: &Path, workspace: Option<&Path>) -> Result<Vec<SessionSummary>> {
    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }
    let mut summaries = Vec::new();
    for entry in std::fs::read_dir(sessions_dir)? {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if is_json(&path)
            && let Some(summary) = parse_summary(&path, workspace)
        {
            summaries.push(summary);
        }
    }
    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

fn is_json(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "json")
}
