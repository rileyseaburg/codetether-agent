//! Sandboxed branch of persistent command creation.

use anyhow::Result;
use std::path::Path;

use super::super::{Running, SpawnMetadata};
use crate::tool::sandbox::SandboxPolicy;

pub(super) async fn command(
    program: &str,
    args: &[String],
    cwd: &Path,
    keep_stdin: bool,
    environment: &[(String, String)],
    policy: &SandboxPolicy,
) -> Result<Running> {
    let spawned = crate::tool::sandbox::sandbox_spawn::spawn(
        program,
        args,
        policy,
        cwd,
        keep_stdin,
        environment,
    )
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
