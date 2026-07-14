//! Foreground program handoff for an attached mux workspace.

use std::path::Path;
use std::process::Stdio;

use anyhow::{Context, Result};

/// Run an explicitly entered command in the active mux window.
pub(super) async fn run(command: &str, workspace: &Path) -> Result<()> {
    let shell = std::env::var_os("SHELL").unwrap_or_else(|| "/bin/sh".into());
    let status = tokio::process::Command::new(shell)
        .args(["-lc", command])
        .current_dir(workspace)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .with_context(|| format!("run command in {}", workspace.display()))?;
    if !status.success() {
        eprintln!("mux: command exited with {status}");
    }
    Ok(())
}

#[cfg(test)]
#[path = "exec_tests.rs"]
mod tests;
