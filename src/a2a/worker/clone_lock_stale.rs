//! Clone lock staleness check.

use std::path::Path;
use std::time::{Duration, SystemTime};

pub(super) async fn workspace_clone_lock_is_stale(
    lock_path: &Path,
    stale_after: Duration,
) -> bool {
    let Ok(metadata) = tokio::fs::metadata(lock_path).await else {
        return false;
    };
    let Ok(modified) = metadata.modified() else {
        return false;
    };
    SystemTime::now()
        .duration_since(modified)
        .map(|age| age > stale_after)
        .unwrap_or(false)
}
