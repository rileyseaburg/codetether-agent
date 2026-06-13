//! Index-first query pipeline for session listings.
//!
//! Pipeline:
//! 1. Try `index.jsonl`; if missing/unreadable, scan and rebuild.
//! 2. Walk `sessions_dir` for `.json` files not in the index, parse
//!    them via [`super::disk_walk::collect_disk_summaries`], append
//!    them, and prune index entries whose backing file is gone.
//! 3. Compact the index when it grows bloated (>= 4 * unique lines).
//! 4. Filter by workspace and sort by `updated_at` desc.

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;

use super::index_file::path as index_path;
use super::index_file_io::read_sync;
use super::summary::SessionSummary;

/// Index-first listing with full rebuild on missing/stale index.
pub(super) async fn list_indexed(
    sessions_dir: PathBuf,
    workspace_dir: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }

    let index_p = index_path().ok();
    // Read the index on the blocking pool — file I/O and per-line JSON
    // parsing must not stall the async executor.
    let loaded = match index_p.clone() {
        Some(p) => tokio::task::spawn_blocking(move || read_sync(&p).ok())
            .await
            .ok()
            .flatten(),
        None => None,
    };
    let (mut from_index, index_lines): (HashMap<String, SessionSummary>, usize) = match loaded {
        Some((map, lines)) if !map.is_empty() => (map, lines),
        _ => {
            return super::rebuild::rebuild_from_scan(sessions_dir, index_p, workspace_dir).await;
        }
    };

    super::merge_disk::merge_disk_into(&mut from_index, &sessions_dir, &index_p, index_lines)
        .await;
    let summaries: Vec<SessionSummary> = from_index.into_values().collect();
    Ok(super::workspace_query::filter_and_sort(
        summaries,
        workspace_dir.as_deref(),
    ))
}
