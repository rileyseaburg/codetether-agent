//! Compare cached and on-disk contents before skipping a snapshot write.
use super::{hash::hash_content, state::store};

/// Returns `true` if `content` is byte-identical to the last successful save
/// for `id` **and** the target file still exists on disk (so eliding the
/// write is safe). Never elides the first save of a session in this process,
/// and never elides when `path` is missing — this keeps the guard correct
/// across data-dir swaps in tests and if a file is deleted underneath us.
/// # Arguments
/// `id` is the snapshot identity; `content` is the proposed data; `path` is the target.
/// # Returns
/// True only when both the remembered hash and current disk bytes match.
/// # Examples
/// ```rust
/// assert!(!codetether_snapshot_io::is_unchanged("unrecorded", b"data", std::path::Path::new("missing-snapshot")));
/// ```
pub fn is_unchanged(id: &str, content: &[u8], path: &std::path::Path) -> bool {
    if !path.exists() {
        return false;
    }
    let hash = hash_content(content);
    let cached = store()
        .lock()
        .map(|guard| guard.get(id) == Some(&hash))
        .unwrap_or(false);
    cached && std::fs::read(path).is_ok_and(|saved| saved == content)
}
