//! Safe paths for spawned-agent manifests and mailbox snapshots.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

pub(crate) fn root() -> Result<PathBuf> {
    crate::config::Config::data_dir()
        .map(|path| path.join("agents"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}

pub(crate) fn manifest(child_id: &str) -> Result<PathBuf> {
    Ok(root()?.join(format!("{}.agent.json", valid(child_id)?)))
}

pub(crate) fn mailbox(child_id: &str) -> Result<PathBuf> {
    Ok(root()?.join(format!("{}.mailbox.json", valid(child_id)?)))
}

pub(crate) fn is_manifest(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".agent.json"))
}

fn valid(id: &str) -> Result<&str> {
    if id.is_empty()
        || id.len() > 128
        || id.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
    {
        bail!("invalid child session id")
    }
    Ok(id)
}
