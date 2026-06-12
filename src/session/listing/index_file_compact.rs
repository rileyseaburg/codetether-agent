//! Atomic compact-rewrite of the session listing index.

use std::path::Path;

use anyhow::Result;
use tokio::fs;

use super::index_file_compact_io::write_compact_sync;
use super::summary::SessionSummary;

/// Rewrite the index with a single, deduped set of summaries (tmp + rename).
pub(super) async fn write_compact_async(path: &Path, summaries: &[SessionSummary]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.ok();
    }
    let final_path = path.to_path_buf();
    let owned: Vec<SessionSummary> = summaries.to_vec();
    tokio::task::spawn_blocking(move || write_compact_sync(&final_path, &owned))
        .await
        .map_err(|e| anyhow::anyhow!("compact task panicked: {e}"))?
}

/// Heuristic: total lines >= 4 * unique ids means the index is bloated.
pub(super) fn should_compact(total_lines: usize, unique_ids: usize) -> bool {
    unique_ids > 0 && total_lines >= unique_ids.saturating_mul(4)
}
