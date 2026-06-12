//! Rebuild the index from a full directory scan.

use std::path::PathBuf;

use anyhow::Result;

use super::index_file_compact::write_compact_async;
use super::scan::scan;
use super::summary::SessionSummary;
use super::workspace_query::filter_and_sort;

/// Scan everything, persist a fresh compacted index, and return the
/// workspace-filtered, sorted summaries.
///
/// The scan is deliberately unfiltered: the index is shared by every
/// workspace, so a rebuild triggered from one workspace's listing must
/// not drop the other workspaces' sessions from it. The workspace
/// filter is applied only to the returned summaries.
pub(super) async fn rebuild_from_scan(
    sessions_dir: PathBuf,
    index_p: Option<PathBuf>,
    workspace_dir: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    let summaries = scan(sessions_dir, None).await?;
    if let Some(p) = index_p.as_ref() {
        let _ = write_compact_async(p, &summaries).await;
    }
    Ok(filter_and_sort(summaries, workspace_dir.as_deref()))
}
