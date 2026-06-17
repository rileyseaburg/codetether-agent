use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use super::record::SessionListingRecord;
use super::summary::SessionSummary;
use super::workspace::matches_workspace;

/// Max session files parsed concurrently on the blocking pool.
const PARSE_CONCURRENCY: usize = 16;

/// Parse session files in parallel on the blocking pool so large/many
/// histories never stall the async runtime (serde walks every byte to count
/// messages, which can be tens of MB per file).
pub(super) async fn parse_all(
    paths: Vec<PathBuf>,
    workspace: Option<PathBuf>,
) -> Vec<SessionSummary> {
    use futures::stream::{self, StreamExt};
    stream::iter(paths)
        .map(|path| {
            let workspace = workspace.clone();
            tokio::task::spawn_blocking(move || parse_summary(&path, workspace.as_deref()))
        })
        .buffer_unordered(PARSE_CONCURRENCY)
        .filter_map(|joined| async move { joined.ok().flatten() })
        .collect()
        .await
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
