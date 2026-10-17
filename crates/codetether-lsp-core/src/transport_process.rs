//! Own the language-server process, including cancellation during initialization.

use anyhow::{Context, Result};
use tokio::process::Command;

#[path = "process_owner.rs"]
mod owner;
#[cfg(all(test, target_os = "linux"))]
#[path = "process_owner_tests.rs"]
mod tests;
pub(super) use owner::ServerProcess;

pub(super) fn spawn(command: &str, args: &[String]) -> Result<ServerProcess> {
    let mut process = Command::new(command);
    // Isolate all workers so eviction cannot leave tsserver/cargo descendants.
    #[cfg(unix)]
    process.process_group(0);
    process
        .args(args)
        .kill_on_drop(true)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map(ServerProcess)
        .with_context(|| format!("Failed to spawn language server '{command}'"))
}
