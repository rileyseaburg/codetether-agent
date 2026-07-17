//! Selection of sandboxed or direct process creation.

use anyhow::{Context, Result};
use std::path::Path;

use super::{Running, SpawnMetadata};
use crate::tool::sandbox::SandboxPolicy;

#[path = "spawn/sandboxed.rs"]
mod sandboxed;

pub(crate) async fn command(
    program: &str,
    args: &[String],
    cwd: &Path,
    keep_stdin: bool,
    environment: &[(String, String)],
    sandbox: Option<&SandboxPolicy>,
) -> Result<Running> {
    if let Some(policy) = sandbox {
        return sandboxed::command(program, args, cwd, keep_stdin, environment, policy).await;
    }
    let mut command = tokio::process::Command::new(program);
    command.args(args).current_dir(cwd);
    crate::tool::bash_noninteractive::configure(&mut command);
    command.envs(environment.iter().cloned());
    let terminal = keep_stdin
        .then(|| crate::tool::command_pty::attach(&mut command))
        .transpose()?;
    let child = command.spawn().context("Failed to spawn command")?;
    Running::new_attached(
        child,
        SpawnMetadata {
            sandboxed: false,
            interactive: keep_stdin,
            redactions: Vec::new(),
            unsafe_fallbacks: Vec::new(),
            cwd: cwd.to_path_buf(),
        },
        terminal,
    )
}
