//! Decode the bounded evidence entries from one writeback file.
use super::writeback_read::MAX_ITEMS_PER_FILE;
use std::path::Path;

pub(super) fn read_items(path: &Path) -> Vec<String> {
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
