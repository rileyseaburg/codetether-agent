//! Detached mux server process construction.

use std::process::{Command, Stdio};

use anyhow::Result;

pub(in crate::mux) fn command(
    name: &str,
    workspace: &std::path::Path,
    token: &str,
) -> Result<Command> {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .args(["mux", "serve", "--session", name, "--directory"])
        .arg(workspace)
        .env(crate::mux::token::BOOTSTRAP_ENV, token)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
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
