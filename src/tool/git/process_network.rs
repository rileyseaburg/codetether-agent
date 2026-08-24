//! Sandboxed Git execution for explicitly network-authorized orchestration.

use anyhow::{Context, Result};
use std::path::Path;

pub(crate) async fn output_networked(
    cwd: &Path,
    args: &[String],
    environment: &[(String, String)],
    mutating: bool,
) -> Result<std::process::Output> {
    let (policy, cwd) = super::process_policy::resolve_network(cwd, mutating, true)?;
    let mut trusted_environment = environment.to_vec();
    trusted_environment.push(("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK".into(), "1".into()));
    let child = crate::tool::sandbox::sandbox_spawn::spawn(
        "git",
        args,
        &policy,
        &cwd,
        false,
        &trusted_environment,
    )
    .await
    .context("failed to spawn network-authorized sandboxed git")?
    .child;
    child
        .wait_with_output()
        .await
        .context("failed to collect sandboxed git output")
}
