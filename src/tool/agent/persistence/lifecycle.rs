//! Durable close and resume transitions for child agents.

use super::{manifest::Lifecycle, manifest_io, manifest_scan, restore};
use crate::tool::agent::store::AgentEntry;
use anyhow::{Result, bail};

pub(in crate::tool::agent) struct ClosedAgent {
    pub name: String,
    pub entry: AgentEntry,
    manifest: super::manifest::Manifest,
}

pub(in crate::tool::agent) async fn close(name: &str, entry: &AgentEntry) -> Result<()> {
    manifest_io::save_lifecycle(name, entry, Lifecycle::Closed).await
}

pub(in crate::tool::agent) async fn prepare_resume(
    owner: Option<&str>,
    target: &str,
) -> Result<ClosedAgent> {
    let manifest = manifest_scan::find(owner, target)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Agent {target} not found"))?;
    if !manifest.supports_current_schema() {
        bail!("Agent {target} uses an unsupported manifest version");
    }
    let entry = restore::load(&manifest).await?;
    let name = manifest.name.clone();
    Ok(ClosedAgent {
        name,
        entry,
        manifest,
    })
}

pub(in crate::tool::agent) async fn activate_resume(closed: ClosedAgent) -> Result<AgentEntry> {
    manifest_io::save_lifecycle(&closed.name, &closed.entry, Lifecycle::Open).await?;
    if let Err(error) =
        restore::register_reserved(closed.manifest.clone(), closed.entry.clone()).await
    {
        manifest_io::save_lifecycle(&closed.name, &closed.entry, Lifecycle::Closed).await?;
        return Err(error);
    }
    Ok(closed.entry)
}

pub(in crate::tool::agent) async fn exists(owner: Option<&str>, name: &str) -> Result<bool> {
    Ok(manifest_scan::find(owner, name).await?.is_some())
}
