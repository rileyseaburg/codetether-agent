//! JSONL input-line filtering for the approval store.

pub(super) fn non_empty(line: std::io::Result<String>) -> Option<std::io::Result<String>> {
    match line {
        Ok(value) if value.trim().is_empty() => None,
        other => Some(other),
    }
}
