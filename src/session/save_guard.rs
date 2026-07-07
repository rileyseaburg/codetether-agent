//! No-op save elision via content hashing.
//!
//! [`Session::save`](super::Session::save) can be called on hot paths (every
//! tool step, keystroke-triggered autosaves). When the serialized bytes are
//! identical to the previous successful write for the same session id, the
//! disk write + atomic rename + journal append + index upsert are all pure
//! overhead. This module tracks the last-saved content hash per session id so
//! `save` can return early when nothing changed.
//!
//! The guard is intentionally conservative: it only elides when it is
//! *certain* the on-disk bytes already match. On process start the map is
//! empty, so the first save of every session always writes.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Mutex, OnceLock};

fn store() -> &'static Mutex<HashMap<String, u64>> {
    static STORE: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Hash the serialized session content.
fn hash_content(content: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

/// Returns `true` if `content` is byte-identical to the last successful save
/// for `id` (i.e. the write can be safely skipped). Never elides the first
/// save of a session in this process.
pub(super) fn is_unchanged(id: &str, content: &[u8]) -> bool {
    let hash = hash_content(content);
    store()
        .lock()
        .map(|guard| guard.get(id) == Some(&hash))
        .unwrap_or(false)
}

/// Record the hash of a successful save so future identical saves elide.
pub(super) fn record_saved(id: &str, content: &[u8]) {
    let hash = hash_content(content);
    if let Ok(mut guard) = store().lock() {
        guard.insert(id.to_string(), hash);
    }
}

/// Forget a session's cached hash (e.g. after deletion) so a later save with
/// the same id always writes.
pub(super) fn forget(id: &str) {
    if let Ok(mut guard) = store().lock() {
        guard.remove(id);
    }
}

/// Atomically write `content` to `path` via a `tmp` sidecar + rename.
///
/// On POSIX `rename` is atomic over an existing file; if it fails (e.g.
/// Windows semantics) we retry once with remove-then-rename.
///
/// # Errors
/// Returns an error only if both the initial rename and the retry fail.
pub(super) async fn atomic_write(
    tmp: &std::path::Path,
    path: &std::path::Path,
    content: &[u8],
) -> anyhow::Result<()> {
    tokio::fs::write(tmp, content).await?;
    if let Err(primary) = tokio::fs::rename(tmp, path).await {
        let _ = tokio::fs::remove_file(path).await;
        if let Err(retry) = tokio::fs::rename(tmp, path).await {
            let _ = tokio::fs::remove_file(tmp).await;
            anyhow::bail!("session rename failed: {primary} (retry: {retry})");
        }
    }
    Ok(())
}
