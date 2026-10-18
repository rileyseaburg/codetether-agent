//! Byte-range metadata encoded in JSONL archive filenames.
use std::path::Path;
pub(crate) fn offsets(path: &Path) -> Option<(u64, u64)> {
    if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
        return None;
    }
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let offsets = filename
        .strip_prefix("T")
        .or_else(|| filename.strip_prefix("202"))?;
    let parts: Vec<&str> = offsets.split('-').collect();
    if parts.len() < 4 {
        return None;
    }
    let start: u64 = parts[parts.len() - 2].parse().unwrap_or(0);
    let end: u64 = parts[parts.len() - 1]
        .trim_end_matches(".jsonl")
        .parse()
        .unwrap_or(0);
    Some((start, end))
}
