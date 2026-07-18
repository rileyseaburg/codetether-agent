//! Rebuild the in-memory child registry from durable manifests.

use super::{manifest_scan, paths, restore};
use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref HYDRATED: Mutex<HashSet<(PathBuf, Option<String>)>> = Mutex::new(HashSet::new());
}

pub(crate) async fn parent(owner: Option<&str>) -> Result<usize> {
    let key = (paths::root()?, owner.map(str::to_string));
    let mut hydrated = HYDRATED.lock().await;
    if hydrated.contains(&key) {
        return Ok(0);
    }
    let mut count = 0;
    for manifest in manifest_scan::all().await? {
        if manifest.owner_session_id.as_deref() == owner {
            match restore::manifest(manifest).await {
                Ok(restored) => count += restored,
                Err(error) => tracing::warn!(%error, "Skipping unrecoverable agent manifest"),
            }
        }
    }
    hydrated.insert(key);
    Ok(count)
}
