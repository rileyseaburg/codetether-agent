//! Rebuild the in-memory child registry from durable manifests.

use crate::{Host, paths, restore, scan as manifest_scan};
use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref HYDRATED: Mutex<HashSet<(PathBuf, Option<String>)>> = Mutex::new(HashSet::new());
}

/// Restore open children for an owner once per storage root and owner scope.
///
/// # Arguments
/// `owner` selects the durable owner scope, including unowned children for `None`.
/// # Returns
/// Returns the number restored; individual unrecoverable manifests are logged and skipped.
/// # Errors
/// Storage root resolution or manifest discovery fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::{Host, hydrate_parent};
/// # async fn example<H: Host>() -> anyhow::Result<()> {
/// let count = hydrate_parent::<H>(Some("parent")).await?;
/// # Ok(()) }
/// ```
pub async fn hydrate_parent<H: Host>(owner: Option<&str>) -> Result<usize> {
    let key = (paths::root()?, owner.map(str::to_string));
    let mut hydrated = HYDRATED.lock().await;
    if hydrated.contains(&key) {
        return Ok(0);
    }
    let mut count = 0;
    for manifest in manifest_scan::all().await? {
        if manifest.owner_session_id.as_deref() == owner {
            match restore::manifest::<H>(manifest).await {
                Ok(restored) => count += restored,
                Err(error) => {
                    tracing::warn!(target: "codetether_agent::tool::agent::persistence::hydrate", %error, "Skipping unrecoverable agent manifest")
                }
            }
        }
    }
    hydrated.insert(key);
    Ok(count)
}
