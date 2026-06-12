//! Merge newly discovered disk summaries into the in-memory index,
//! prune entries whose backing file is gone, and compact the on-disk
//! index when it has accumulated too many superseded lines.

use std::collections::HashMap;
use std::path::Path;

use super::disk_walk;
use super::index_file::append_async;
use super::index_file_compact::{should_compact, write_compact_async};
use super::index_prune;
use super::summary::SessionSummary;

/// `index_lines` is the raw line count of `index.jsonl` as read, used
/// together with the appends performed here to drive the compaction
/// heuristic.
pub(super) async fn merge_disk_into(
    from_index: &mut HashMap<String, SessionSummary>,
    sessions_dir: &Path,
    index_p: &Option<std::path::PathBuf>,
    index_lines: usize,
) {
    let disk = disk_walk::collect_disk_summaries(sessions_dir, from_index).await;
    let mut appended = 0usize;
    for s in disk {
        if !from_index.contains_key(&s.id) {
            append_async(&s).await;
            from_index.insert(s.id.clone(), s);
            appended += 1;
        }
    }
    index_prune::retain_existing(from_index, sessions_dir);

    if let Some(p) = index_p.as_ref() {
        let unique = from_index.len();
        if should_compact(index_lines + appended, unique) {
            let values: Vec<SessionSummary> = from_index.values().cloned().collect();
            let _ = write_compact_async(p, &values).await;
        }
    }
}
