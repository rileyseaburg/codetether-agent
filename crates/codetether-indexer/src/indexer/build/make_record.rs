//! make record for index construction.

use super::*;

pub(super) fn make_record(
    path: &Path,
    rel_path: &Path,
    metadata: &std::fs::Metadata,
    content: &str,
) -> IndexedFile {
    let language = detect_language(path);
    let lines = if content.is_empty() {
        0
    } else {
        (content.as_bytes().iter().filter(|b| **b == b'\n').count() + 1) as u32
    };
    let symbol_hints = estimate_symbol_hints(path, &content);

    let rel_path = rel_path.to_string_lossy().to_string();

    let modified_unix_ms = metadata
        .modified()
        .ok()
        .and_then(|ts| ts.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|dur| dur.as_millis() as i64);

    IndexedFile {
        path: rel_path.clone(),
        language: language.clone(),
        bytes: metadata.len(),
        lines,
        symbol_hints,
        modified_unix_ms,
        embedding: Vec::new(),
    }
}
