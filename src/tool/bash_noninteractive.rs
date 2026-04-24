//! Noninteractive subprocess hardening for shell-backed tools.

use std::process::Stdio;
use tokio::process::Command;

pub(super) fn configure(cmd: &mut Command) {
    configure_stdio(cmd);
    configure_auth_env(cmd);
    detach_controlling_terminal(cmd);
}

fn configure_stdio(cmd: &mut Command) {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
}

fn configure_auth_env(cmd: &mut Command) {
    cmd.env("GIT_TERMINAL_PROMPT", "0")
        .env("GCM_INTERACTIVE", "never")
        .env("DEBIAN_FRONTEND", "noninteractive")
        .env("SUDO_ASKPASS", "/bin/false")
        .env("SSH_ASKPASS", "/bin/false");
}

#[cfg(unix)]
fn detach_controlling_terminal(cmd: &mut Command) {
    // SAFETY: `pre_exec` runs after fork in the child. The closure only calls
    // `setsid` and converts errno, which keeps it async-signal-safe enough for
    // this child-process setup hook.
    unsafe {
        cmd.pre_exec(|| {
            if libc::setsid() == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
}

#[cfg(not(unix))]
fn detach_controlling_terminal(_cmd: &mut Command) {}
