//! Filesystem and network policy for internal Git subprocesses.

#[path = "process_policy_metadata.rs"]
mod metadata;

use crate::tool::sandbox::SandboxPolicy;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub(super) fn resolve(cwd: &Path, mutating: bool) -> Result<(SandboxPolicy, PathBuf)> {
    resolve_network(cwd, mutating, false)
}

pub(super) fn resolve_network(
    cwd: &Path,
    mutating: bool,
    allow_network: bool,
) -> Result<(SandboxPolicy, PathBuf)> {
    let cwd = cwd.canonicalize().context("invalid git workspace")?;
    let mut policy = SandboxPolicy {
        allow_exec: true,
        allow_network,
        ..Default::default()
    };
    if mutating {
        policy.allowed_paths.extend(metadata::writable(&cwd));
        policy.protect_metadata = false;
    }
    Ok((policy, cwd))
}
