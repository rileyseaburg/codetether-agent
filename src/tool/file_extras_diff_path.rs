//! Workspace confinement for external diff process paths.

use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::path::{Component, Path, PathBuf};

pub(super) fn root(args: &Value) -> Result<PathBuf> {
    crate::tool::network_access::trusted_workspace(args)
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?)
        .canonicalize()
        .context("invalid diff workspace")
}

pub(super) fn confined(root: &Path, raw: &str) -> Result<String> {
    relative(raw)?;
    let path = root.join(raw).canonicalize()?;
    if !path.starts_with(root) {
        bail!("diff path is outside workspace")
    }
    Ok(path.display().to_string())
}

pub(super) fn relative(raw: &str) -> Result<()> {
    if Path::new(raw)
        .components()
        .any(|part| !matches!(part, Component::Normal(_) | Component::CurDir))
    {
        bail!("diff path must be workspace-relative")
    }
    Ok(())
}
