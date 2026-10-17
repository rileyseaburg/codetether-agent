//! Selection of sandboxed or direct process creation.

use anyhow::Result;
use std::path::Path;

use super::Running;
use codetether_sandbox_policy::SandboxPolicy;

#[path = "create/sandboxed.rs"]
mod sandboxed;

/// Create a persistent session using the requested sandbox policy.
///
/// # Arguments
/// The executable, arguments, working directory, interactive flag and environment
/// define the child. A supplied sandbox policy selects sandboxed creation.
/// # Returns
/// A running session with the selected transport and execution metadata.
/// # Errors
/// Returns validation, planning, transport or spawning errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _session = codetether_command_session::spawn(
///     "example", &[], std::path::Path::new("."), false, &[], None,
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn spawn(
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
    crate::spawn_direct(program, args, cwd, keep_stdin, environment).await
}
