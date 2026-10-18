//! Record successful writes and forget removed snapshots.
use super::{hash::hash_content, state::store};

/// Record the hash of a successful save so future identical saves elide.
/// # Arguments
/// `id` is the snapshot identity; `content` must represent a successful write.
/// # Returns
/// Unit; a poisoned cache is ignored, so later reads conservatively write again.
/// # Examples
/// ```rust
/// codetether_snapshot_io::record_saved("example", b"saved bytes");
/// codetether_snapshot_io::forget("example");
/// ```
pub fn record_saved(id: &str, content: &[u8]) {
    let hash = hash_content(content);
    if let Ok(mut guard) = store().lock() {
        guard.insert(id.to_string(), hash);
    }
}

/// Forget a session's cached hash (e.g. after deletion) so a later save with
/// the same id always writes.
/// # Arguments
/// `id` identifies the snapshot whose successful-write hash is obsolete.
/// # Returns
/// Unit, including when the identity is absent or cache synchronization failed.
/// # Examples
/// ```rust
/// codetether_snapshot_io::forget("example");
/// ```
pub fn forget(id: &str) {
    if let Ok(mut guard) = store().lock() {
        guard.remove(id);
    }
}
