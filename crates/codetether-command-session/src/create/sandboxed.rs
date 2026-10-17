//! Sandboxed branch of persistent command creation.

use anyhow::Result;
use std::path::Path;

use super::super::{Running, SpawnMetadata};
use codetether_sandbox_policy::SandboxPolicy;

pub(super) async fn command(
    program: &str,
    args: &[String],
    cwd: &Path,
    keep_stdin: bool,
    environment: &[(String, String)],
    policy: &SandboxPolicy,
) -> Result<Running> {
    let spawned =
        codetether_sandbox_runtime::spawn(program, args, policy, cwd, keep_stdin, environment)
            .await?;
    Running::new_attached(
        spawned.child,
        SpawnMetadata {
            sandboxed: true,
            interactive: keep_stdin,
            redactions: Vec::new(),
            unsafe_fallbacks: spawned.unsafe_fallbacks,
            cwd: cwd.to_path_buf(),
        },
        spawned.terminal,
    )
}
