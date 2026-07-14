use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;

use super::cache::ListingCache;
use super::cache_io::{load, store};
use super::resolve::resolve_all;
use super::summary::SessionSummary;
use super::workspace::matches_workspace;
use crate::session::workspace_index::INDEX_FILENAME;

pub(super) async fn scan(
    sessions_dir: PathBuf,
    workspace: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }
    let paths = collect_json_paths(&sessions_dir).await?;
    let cache = load(&sessions_dir).await;
    let resolved = resolve_all(paths, &cache).await;

    let mut next = ListingCache::default();
    let mut summaries = Vec::with_capacity(resolved.len());
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

async fn collect_json_paths(sessions_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let mut entries = fs::read_dir(sessions_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if is_session_json(&path) {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn is_session_json(path: &Path) -> bool {
    let name = path.file_name().and_then(|name| name.to_str());
    if name == Some(".listing_cache.json") || name == Some(INDEX_FILENAME) {
        return false;
    }
    path.extension().is_some_and(|ext| ext == "json")
}
