//! Captured output from a short-lived sandboxed subprocess.

use super::SandboxPolicy;
use anyhow::Result;
use std::path::Path;

pub(crate) async fn run(
    program: &str,
    args: &[String],
    policy: &SandboxPolicy,
    cwd: &Path,
) -> Result<std::process::Output> {
    let spawned = super::sandbox_spawn::spawn(program, args, policy, cwd, false, &[]).await?;
    Ok(spawned.child.wait_with_output().await?)
}