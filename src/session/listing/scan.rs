use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;

use super::record::SessionListingRecord;
use super::summary::SessionSummary;
use super::workspace::matches_workspace;

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

fn parse_summary(path: &Path, workspace: Option<&Path>) -> Option<SessionSummary> {
    let file = File::open(path).map_err(log_read(path)).ok()?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    let record: SessionListingRecord = serde_json::from_reader(reader)
        .map_err(log_malformed(path))
        .ok()?;
    let summary = record.into_summary();
    matches_workspace(summary.directory.as_deref(), workspace).then_some(summary)
}

fn log_read(path: &Path) -> impl FnOnce(std::io::Error) {
    |error| tracing::warn!(path = %path.display(), error = %error, "skipping unreadable session file")
}

fn log_malformed(path: &Path) -> impl FnOnce(serde_json::Error) {
    |error| tracing::warn!(path = %path.display(), error = %error, "skipping malformed session file")
}
