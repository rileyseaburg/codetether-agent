//! Reads recent evidence writeback files and summarises them for prompt
//! injection, closing the loop between `writeback_persist` and future turns.
//!
//! Reads the most recent N writeback JSON files from
//! `.codetether/memory-writeback/` and renders a compact digest so the model
//! sees what was proven in prior sessions without relying on full recall.

use std::path::Path;

/// Maximum writeback files to scan per prompt build.
const MAX_RECENT: usize = 5;

/// Maximum evidence items to surface per file.
const MAX_ITEMS_PER_FILE: usize = 3;

/// Render a digest of recent evidence writebacks for `cwd`.
pub(crate) fn render(cwd: &Path) -> String {
    let dir = cwd.join(".codetether").join("memory-writeback");
    if !dir.exists() {
        return String::new();
    }
    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "json"))
        .collect();
    entries.sort_by_key(|e| std::cmp::Reverse(e.file_name()));
    let items: Vec<String> = entries
        .iter()
        .take(MAX_RECENT)
        .flat_map(|e| read_items(&e.path()))
        .take(MAX_ITEMS_PER_FILE * MAX_RECENT)
        .collect();
    if items.is_empty() {
        return String::new();
    }
    format!("Recent proven deliverables:\n{}", items.join("\n"))
}

fn read_items(path: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&text) else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|v| {
            let level = v["level"].as_str()?;
            let value = v["value"].as_str()?;
            Some(format!("- [{level}] {value}"))
        })
        .take(MAX_ITEMS_PER_FILE)
        .collect()
}
