//! Sandboxed GitHub CLI credential process boundary.

use anyhow::{Context, Result};
use std::io::Write;

pub(super) fn run(token: &str, payload: &str) -> Result<std::process::Output> {
    let cwd = std::env::current_dir().context("Failed to resolve credential workspace")?;
    let args = vec!["auth".into(), "git-credential".into(), "get".into()];
    let policy = crate::tool::sandbox::SandboxPolicy {
        allow_exec: true,
        allow_network: false,
        ..Default::default()
    };
    let environment = vec![("GH_TOKEN".into(), token.to_string())];
    let mut child = crate::tool::sandbox::sandbox_spawn_std::spawn_with_env_input(
        "gh",
        &args,
        &policy,
        &cwd,
        &environment,
    )
    .context("Failed to spawn gh auth git-credential")?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(payload.as_bytes())
            .context("Failed to write Git credential request to gh")?;
    }
    child
        .wait_with_output()
        .context("Failed to read gh auth git-credential output")
}
