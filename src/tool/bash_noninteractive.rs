//! Noninteractive subprocess hardening for shell-backed tools.

use std::process::Stdio;
use tokio::process::Command;

pub(super) fn configure(cmd: &mut Command) {
    configure_stdio(cmd);
    configure_auth_env(cmd);
    super::process_tree::configure(cmd);
}

fn configure_stdio(cmd: &mut Command) {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
}

fn configure_auth_env(cmd: &mut Command) {
    cmd.env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_ASKPASS", "/bin/false")
        .env("GCM_INTERACTIVE", "never")
        .env("DEBIAN_FRONTEND", "noninteractive")
        .env("SUDO_ASKPASS", "/bin/false")
        .env("SSH_ASKPASS", "/bin/false");
}
