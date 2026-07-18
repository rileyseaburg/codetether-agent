//! Atomic persistence for durable child manifests.

use super::{
    atomic,
    manifest::{Lifecycle, Manifest},
    paths,
};
use crate::tool::agent::store::AgentEntry;
use anyhow::Result;

pub(in crate::tool::agent) async fn save(name: &str, entry: &AgentEntry) -> Result<()> {
    atomic::write(
        &paths::manifest(&entry.session.id)?,
        &Manifest::from_entry(name, entry),
    )
    .await
}

pub(in crate::tool::agent) async fn save_lifecycle(
    name: &str,
    entry: &AgentEntry,
    lifecycle: Lifecycle,
) -> Result<()> {
    let manifest = Manifest::from_entry(name, entry).with_lifecycle(lifecycle);
    atomic::write(&paths::manifest(&entry.session.id)?, &manifest).await
}

pub(in crate::tool::agent) async fn remove(entry: &AgentEntry) -> Result<()> {
    atomic::remove(&paths::manifest(&entry.session.id)?).await
}
