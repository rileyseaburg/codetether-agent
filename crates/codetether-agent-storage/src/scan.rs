//! Resilient discovery and lookup of durable child manifests.

use super::{atomic, manifest::Manifest, paths};
use anyhow::Result;

/// Discover durable manifests, logging and skipping invalid sidecars.
/// # Returns
/// All decoded manifest records; an absent storage directory yields an empty list.
/// # Errors
/// Returns directory lookup/read errors.
/// # Examples
/// ```no_run
/// # async fn example()->anyhow::Result<()> {
/// let _manifests=codetether_agent_storage::scan::all().await?;
/// # Ok(()) }
/// ```
pub async fn all() -> Result<Vec<Manifest>> {
    let root = paths::root()?;
    let mut dir = match tokio::fs::read_dir(&root).await {
        Ok(dir) => dir,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let mut manifests = Vec::new();
    while let Some(entry) = dir.next_entry().await? {
        if !paths::is_manifest(&entry.path()) {
            continue;
        }
        match atomic::read(&entry.path()).await {
            Ok(Some(manifest)) => manifests.push(manifest),
            Ok(None) => {}
            Err(error) => {
                tracing::warn!(target:"codetether_agent::tool::agent::persistence::manifest_scan",
                    path = %entry.path().display(), %error, "Skipping invalid agent manifest"
                )
            }
        }
    }
    Ok(manifests)
}

/// Find an owned durable child by identity, name or canonical tree path.
/// # Arguments
/// * `owner` — Optional calling parent identity.
/// * `target` — Identity, local name or tree path to resolve.
/// # Returns
/// The matching manifest within the permitted tree, or none.
/// # Errors
/// Returns manifest discovery errors.
/// # Examples
/// ```no_run
/// # async fn example()->anyhow::Result<()> {
/// let _child=codetether_agent_storage::scan::find(Some("parent"),"worker").await?;
/// # Ok(()) }
/// ```
pub async fn find(owner: Option<&str>, target: &str) -> Result<Option<Manifest>> {
    let manifests = all().await?;
    let direct = manifests.iter().find(|manifest| {
        manifest.owner_session_id.as_deref() == owner
            && (manifest.child_session_id == target || manifest.name == target)
    });
    if let Some(direct) = direct {
        return Ok(Some(direct.clone()));
    }
    Ok(owner.and_then(|owner| super::tree_target::find(&manifests, owner, target)))
}
