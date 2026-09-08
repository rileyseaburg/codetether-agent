//! Own the language-server process, including cancellation during initialization.

use anyhow::{Context, Result};
use tokio::process::{Child, Command};

pub(super) fn spawn(command: &str, args: &[String]) -> Result<Child> {
    Command::new(command)
        .args(args)
        .kill_on_drop(true)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn language server '{command}'"))
}
