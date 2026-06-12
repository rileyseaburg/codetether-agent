use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;

use super::parse_summary::parse_summary;
use super::summary::SessionSummary;

/// Full-directory listing scan. Used as the rebuild/repair path when
/// `index.jsonl` is missing or stale.
pub(super) async fn scan(
    sessions_dir: PathBuf,
    workspace: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }
    let mut summaries = Vec::new();
    let mut entries = fs::read_dir(&sessions_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if is_json(&path)
            && let Some(summary) = parse_summary(&path, workspace.as_deref())
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
