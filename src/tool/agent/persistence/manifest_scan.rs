//! Resilient discovery and lookup of durable child manifests.

use super::{atomic, manifest::Manifest, paths};
use anyhow::Result;

pub(super) async fn all() -> Result<Vec<Manifest>> {
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
            Err(error) => tracing::warn!(
                path = %entry.path().display(), %error, "Skipping invalid agent manifest"
            ),
        }
    }
    Ok(manifests)
}

pub(super) async fn find(owner: Option<&str>, target: &str) -> Result<Option<Manifest>> {
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
