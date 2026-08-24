//! Workspace and network confinement for MCP subprocess transports.

use crate::tool::sandbox::SandboxPolicy;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub(crate) fn policy(allow_network: bool) -> Result<(SandboxPolicy, PathBuf)> {
    let cwd = std::env::current_dir()
        .context("failed to resolve MCP subprocess workspace")?
        .canonicalize()
        .context("failed to canonicalize MCP subprocess workspace")?;
    let policy = SandboxPolicy {
        allowed_paths: vec![cwd.clone()],
        allow_network,
        allow_exec: true,
        timeout_secs: 0,
        ..SandboxPolicy::default()
    };
    Ok((policy, cwd))
}

pub(crate) async fn preflight(command: &str, args: &[&str], allow_network: bool) -> Result<()> {
    let (policy, cwd) = policy(allow_network)?;
    let args = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    crate::tool::sandbox::sandbox_preflight::validate(command, &args, &policy, &cwd).await
}

#[cfg(test)]
#[path = "subprocess_sandbox_tests.rs"]
mod tests;
