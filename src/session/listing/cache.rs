//! Disk-backed session listing cache keyed on file (mtime, size).
//!
//! Parsing every session file on each picker open is `O(bytes)` — serde walks
//! the entire `messages` array (tens of MB) just to count messages. This cache
//! lets unchanged files skip parsing entirely: a `stat` is effectively free.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::summary::SessionSummary;

/// One cached listing entry validated against the file's `(mtime_ns, size)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct CacheEntry {
    pub(super) mtime_ns: u128,
    pub(super) size: u64,
    pub(super) summary: SessionSummary,
}

/// Path-keyed listing cache persisted next to the session files.
#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct ListingCache {
    #[serde(default)]
    pub(super) entries: HashMap<String, CacheEntry>,
}

impl ListingCache {
    /// Reuse a cached summary when the file's `(mtime, size)` is unchanged.
    pub(super) fn hit(&self, path: &Path, mtime_ns: u128, size: u64) -> Option<SessionSummary> {
        let entry = self.entries.get(path.to_str()?)?;
        (entry.mtime_ns == mtime_ns && entry.size == size).then(|| entry.summary.clone())
    }
}

/// Location of the cache file inside the sessions directory.
pub(super) fn cache_path(sessions_dir: &Path) -> PathBuf {
    sessions_dir.join(".listing_cache.json")
}
