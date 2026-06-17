use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;

use super::parse::parse_all;
use super::summary::SessionSummary;

pub(super) async fn scan(
    sessions_dir: PathBuf,
    workspace: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    let mut entries = fs::read_dir(&sessions_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if is_json(&path) {
            paths.push(path);
        }
    }
    let mut summaries = parse_all(paths, workspace).await;
    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

fn is_json(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "json")
}
