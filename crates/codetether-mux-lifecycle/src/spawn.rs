//! Detached mux server process construction.

use std::process::{Command, Stdio};

use anyhow::Result;

/// Construct a detached mux daemon command using the current executable.
///
/// # Arguments
/// name and workspace identify the initial session; token is its bootstrap credential; isolation controls workspace allocation.
/// # Returns
/// An unstarted process command with detached standard streams.
/// # Errors
/// Returns failure to resolve the current executable.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let command = codetether_mux_lifecycle::spawn::command("work", std::path::Path::new("."), "bootstrap", codetether_mux_workspace::Isolation::Shared)?;
/// # Ok(()) }
/// ```
pub fn command(
    name: &str,
    workspace: &std::path::Path,
    token: &str,
    isolation: codetether_mux_workspace::Isolation,
) -> Result<Command> {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .args(["mux", "serve", "--session", name, "--directory"])
        .arg(workspace)
        .env(codetether_mux_server::token::BOOTSTRAP_ENV, token)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if isolation.is_shared() {
        command.arg("--no-worktree");
    }
    isolate(&mut command);
    Ok(command)
}

#[cfg(unix)]
fn isolate(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    // SAFETY: this hook runs after fork and only calls the async-signal-safe
    // `setsid`, ensuring the mux daemon does not retain the client's terminal.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
}

#[cfg(not(unix))]
fn isolate(_: &mut Command) {}
