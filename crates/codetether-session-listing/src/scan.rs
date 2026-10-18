use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;

use super::cache::ListingCache;
use super::cache_io::{load, store};
use super::resolve::resolve_all;
use super::summary::SessionSummary;
use super::workspace::matches_workspace;

/// Combine indexed summaries and cached legacy JSON records, newest first.
///
/// # Arguments
/// * `sessions_dir` — Directory containing the index and legacy snapshots.
/// * `workspace` — Optional workspace filter, canonicalized by the caller.
/// # Returns
/// Summaries sorted by descending update time.
/// # Errors
/// Propagates directory enumeration and indexed-source failures.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_listing::Source>() -> anyhow::Result<()> {
/// let rows = codetether_session_listing::scan::<S>(S::sessions_dir()?, None).await?;
/// assert!(rows.windows(2).all(|w| w[0].updated_at >= w[1].updated_at));
/// # Ok(()) }
/// ```
pub async fn scan<S: crate::Source>(
    sessions_dir: PathBuf,
    workspace: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }
    let paths = collect_json_paths::<S>(&sessions_dir).await?;
    let cache = load(&sessions_dir).await;
    let resolved = resolve_all(paths, &cache).await;

    let mut next = ListingCache::default();
    let mut summaries = S::indexed_sessions(&sessions_dir, workspace.clone()).await?;
    for (key, entry) in resolved {
        if matches_workspace(entry.summary.directory.as_deref(), workspace.as_deref()) {
            summaries.push(entry.summary.clone());
        }
        next.entries.insert(key, entry);
    }
    store(&sessions_dir, &next).await;

    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

async fn collect_json_paths<S: crate::Source>(sessions_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let mut entries = fs::read_dir(sessions_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if super::path::is_session_json(&path, S::INDEX_FILENAME) {
            paths.push(path);
        }
    }
    Ok(paths)
}
