//! Read the sessions directory and parse `.json` files not yet in the
//! index. See [`super::index_prune`] for the dead-entry reaper.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use super::parse_summary::parse_summary_unfiltered;
use super::summary::SessionSummary;

/// Walk `sessions_dir` and parse any `.json` whose id (filename stem) is
/// not already known to the index. Non-session json files (e.g.,
/// `.workspace_index.json`) are excluded by the id-shape check.
///
/// Returns the newly parsed summaries plus the full set of session ids
/// present on disk, so the caller can prune dead index entries without
/// any further disk I/O. The walk and parsing run on the blocking pool —
/// readdir, file reads, and JSON parsing must not stall the async
/// executor.
pub(super) async fn collect_disk_summaries(
    sessions_dir: &Path,
    known: &HashMap<String, SessionSummary>,
) -> (Vec<SessionSummary>, HashSet<String>) {
    let dir = sessions_dir.to_path_buf();
    let known_ids: HashSet<String> = known.keys().cloned().collect();
    tokio::task::spawn_blocking(move || collect_sync(&dir, &known_ids))
        .await
        .unwrap_or_default()
}

fn collect_sync(
    sessions_dir: &Path,
    known: &HashSet<String>,
) -> (Vec<SessionSummary>, HashSet<String>) {
    let mut out = Vec::new();
    let mut present: HashSet<String> = HashSet::new();
    let Ok(entries) = std::fs::read_dir(sessions_dir) else {
        return (out, present);
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !is_session_json(&path) {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if !is_valid_id_shape(stem) {
            continue;
        }
        present.insert(stem.to_string());
        if known.contains(stem) {
            continue;
        }
        if let Some(summary) = parse_summary_unfiltered(&path) {
            out.push(summary);
        }
    }
    (out, present)
}

fn is_session_json(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("json")
}

fn is_valid_id_shape(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
