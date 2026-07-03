//! Cache-aware resolution of session files into summaries.
//!
//! For each file: `stat` for `(mtime, size)`, reuse the cached summary on a
//! hit, otherwise parse on the blocking pool. Unchanged files never re-parse.

use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use futures::stream::{self, StreamExt};

use super::cache::{CacheEntry, ListingCache};
use super::parse::parse_summary;
use super::summary::SessionSummary;

/// Max session files resolved concurrently on the blocking pool.
const RESOLVE_CONCURRENCY: usize = 16;

/// Resolve every path to `(path, entry)`, using the cache where possible.
pub(super) async fn resolve_all(
    paths: Vec<PathBuf>,
    cache: &ListingCache,
) -> Vec<(String, CacheEntry)> {
    stream::iter(paths)
        .map(|path| resolve_one(path, cache))
        .buffer_unordered(RESOLVE_CONCURRENCY)
        .filter_map(|entry| async move { entry })
        .collect()
        .await
}

async fn resolve_one(path: PathBuf, cache: &ListingCache) -> Option<(String, CacheEntry)> {
    let meta = tokio::fs::metadata(&path).await.ok()?;
    let size = meta.len();
    let mtime_ns = meta
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_nanos();
    let key = path.to_str()?.to_string();
    let summary = match cache.hit(&path, mtime_ns, size) {
        Some(summary) => summary,
        None => tokio::task::spawn_blocking(move || parse_summary(&path))
            .await
            .ok()??,
    };
    Some((key, entry(mtime_ns, size, summary)))
}

fn entry(mtime_ns: u128, size: u64, summary: SessionSummary) -> CacheEntry {
    CacheEntry {
        mtime_ns,
        size,
        summary,
    }
}
