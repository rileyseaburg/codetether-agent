//! Sandboxed process policy for TUI pull-request publication.

use crate::tool::sandbox::{SandboxPolicy, sandbox_spawn};
use anyhow::{Context, Result};
use std::path::Path;

pub(super) fn require_network(allowed: bool) -> Result<()> {
    if !allowed {
        anyhow::bail!("network access is disabled for pull-request publication");
    }
    Ok(())
}

pub(super) async fn gh(cwd: &Path, args: &[String]) -> Result<std::process::Output> {
    let policy = SandboxPolicy {
        allowed_paths: vec![cwd.to_path_buf()],
        allow_network: true,
        allow_exec: true,
        timeout_secs: 120,
        ..SandboxPolicy::default()
    };
    let environment = std::env::var("GH_TOKEN")
        .ok()
        .map(|token| vec![("GH_TOKEN".into(), token)])
        .unwrap_or_default();
    let child = sandbox_spawn::spawn("gh", args, &policy, cwd, false, &environment)
        .await
        .context("Failed to run sandboxed gh pr create")?
        .child;
    child
        .wait_with_output()
        .await
        .context("Failed to collect gh output")
}

#[cfg(test)]
#[test]
fn publication_requires_session_network_policy() {
    assert!(require_network(false).is_err());
    assert!(require_network(true).is_ok());
}
