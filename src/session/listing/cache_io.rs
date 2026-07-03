//! Load/store helpers for the session listing cache.

use std::path::Path;

use tokio::fs;

use super::cache::{ListingCache, cache_path};

/// Load the listing cache, returning an empty cache on any error.
pub(super) async fn load(sessions_dir: &Path) -> ListingCache {
    let path = cache_path(sessions_dir);
    match fs::read(&path).await {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => ListingCache::default(),
    }
}

/// Persist the listing cache; failures are logged and ignored (best-effort).
pub(super) async fn store(sessions_dir: &Path, cache: &ListingCache) {
    let path = cache_path(sessions_dir);
    let Ok(bytes) = serde_json::to_vec(cache) else {
        return;
    };
    if let Err(error) = fs::write(&path, bytes).await {
        tracing::warn!(path = %path.display(), error = %error, "failed to write listing cache");
    }
}
