//! SSH invocation builder for `codetether connect`.
//!
//! Builds an `ssh` command that forwards a dedicated port both ways and runs
//! the remote `codetether auth` device-code flow with stdout streamed back.

use super::args::ConnectArgs;
use tokio::process::Command;

/// Build the `ssh` [`Command`] that forwards the dedicated port and runs the
/// remote auth flow. stdout is piped so the caller can scan for the URL.
pub fn build(args: &ConnectArgs) -> Command {
    let forward = forward_spec(args.forward_port);
    let mut cmd = Command::new("ssh");
    cmd.args(["-t", "-p", &args.port.to_string()]);
    // Forward the dedicated callback port both directions for device-code.
    cmd.args(["-L", &forward, "-R", &forward]);
    cmd.arg(target(args));
    cmd.arg(remote_command(args));
    cmd.stdout(std::process::Stdio::piped());
    cmd
}

/// `user@host` when a user is set, otherwise the bare host.
pub(super) fn target(args: &ConnectArgs) -> String {
    match &args.user {
        Some(user) => format!("{user}@{}", args.host),
        None => args.host.clone(),
    }
}

/// Both-directions port forward spec bound to loopback on each side.
pub(super) fn forward_spec(port: u16) -> String {
    format!("{port}:127.0.0.1:{port}")
}

/// Compose the remote shell command string run on the VM.
pub(super) fn remote_command(args: &ConnectArgs) -> String {
    let mut parts = vec![
        args.remote_bin.clone(),
        "auth".into(),
        args.provider.clone(),
        "--device-code".into(),
    ];
    parts.extend(args.remote_args.iter().cloned());
    parts.join(" ")
}
