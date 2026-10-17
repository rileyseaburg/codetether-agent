//! Direct process creation for persistent command sessions.
use crate::{Running, SpawnMetadata};
use anyhow::{Context, Result};
use std::path::Path;
/// Spawn a direct command with the standard noninteractive setup.
///
/// # Arguments
/// * `program` - Executable name or path.
/// * `args` - Arguments following the executable.
/// * `cwd` - Child working directory.
/// * `keep_stdin` - Attach an interactive terminal when true.
/// * `environment` - Environment overrides applied after standard setup.
///
/// # Returns
/// A running session ready for input and incremental output polling.
///
/// # Errors
/// Returns an error if terminal allocation, spawning or attachment fails.
///
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _session = codetether_command_session::spawn_direct(
///     "example", &[], std::path::Path::new("."), false, &[],
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn spawn_direct(
    program: &str,
    args: &[String],
    cwd: &Path,
    keep_stdin: bool,
    environment: &[(String, String)],
) -> Result<Running> {
    let mut command = tokio::process::Command::new(program);
    command.args(args).current_dir(cwd);
    codetether_command_shell::configure(&mut command);
    command.envs(environment.iter().cloned());
    let terminal = keep_stdin
        .then(|| codetether_command_pty::attach(&mut command))
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
