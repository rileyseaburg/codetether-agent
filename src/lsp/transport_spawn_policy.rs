//! Read-only runtime exposure for sandboxed language-server executables.

use crate::tool::sandbox::SandboxPolicy;
use anyhow::{Context, Result};

#[path = "transport_spawn_rust.rs"]
mod rust;

pub(super) fn resolve(command: &str) -> Result<(String, SandboxPolicy)> {
    let executable = which::which(command)
        .with_context(|| format!("language server '{command}' is not on PATH"))?;
    let mut policy = SandboxPolicy {
        allow_exec: true,
        allow_network: false,
        ..Default::default()
    };
    if let Some(bin) = executable.parent() {
        policy.read_only_paths.push(bin.to_path_buf());
        if command == "rust-analyzer" {
            rust::add(&mut policy, bin);
        }
    }
    Ok((executable.display().to_string(), policy))
}

#[cfg(test)]
#[path = "transport_spawn_policy_tests.rs"]
mod tests;
