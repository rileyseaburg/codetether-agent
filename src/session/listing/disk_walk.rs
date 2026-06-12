//! Read the sessions directory and parse `.json` files not yet in the
//! index. See [`super::index_prune`] for the dead-entry reaper.

use std::collections::HashMap;
use std::path::Path;

use tokio::fs;

use super::parse_summary::parse_summary_unfiltered;
use super::summary::SessionSummary;

/// Walk `sessions_dir` and parse any `.json` whose id (filename stem) is
/// not already known to the index. Non-session json files (e.g.,
/// `.workspace_index.json`) are excluded by the id-shape check.
pub(super) async fn collect_disk_summaries(
    sessions_dir: &Path,
    known: &HashMap<String, SessionSummary>,
) -> Vec<SessionSummary> {
    let mut entries = match fs::read_dir(sessions_dir).await {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if !is_session_json(&path) {
            continue;
        }
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        if !is_valid_id_shape(&stem) || known.contains_key(&stem) {
            continue;
        }
        if let Some(summary) = parse_summary_unfiltered(&path) {
            out.push(summary);
        }
    }
    out
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
