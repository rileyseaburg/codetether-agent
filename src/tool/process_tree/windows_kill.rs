//! Fallback Windows process-tree termination.
use std::os::windows::process::CommandExt;
use std::process::{Command as StdCommand, Stdio};
/// Kills a command tree even when job assignment is unavailable.
pub(super) fn terminate_with_taskkill(pid: u32) {
    let status = StdCommand::new("taskkill.exe")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .creation_flags(0x0800_0000)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if let Err(error) = status {
        tracing::warn!(pid, error = %error, "Failed to terminate cancelled process tree");
    }
}
