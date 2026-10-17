//! Selection of sandboxed or direct process creation.

use anyhow::Result;
use std::path::Path;

use super::Running;
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
    codetether_command_session::spawn_direct(program, args, cwd, keep_stdin, environment).await
}
