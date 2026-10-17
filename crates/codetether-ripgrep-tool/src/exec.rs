//! Spawn ripgrep and collect its output under a wall-clock budget.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;

/// Raw ripgrep outcome, before formatting.
pub(super) struct Output {
    pub stdout: String,
    pub stderr: String,
    /// `None` when the search exceeded its wall-clock budget.
    pub code: Option<i32>,
}

/// Run `rg` with `flags` in `cwd`, aborting after `timeout`.
///
/// # Errors
///
/// Returns an error when the `rg` binary cannot be spawned.
pub(super) async fn run(flags: &[String], cwd: &Path, timeout: Duration) -> Result<Output> {
    let child = Command::new("rg")
        .args(flags)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("failed to spawn `rg`; is ripgrep installed and on PATH?")?;
    match tokio::time::timeout(timeout, child.wait_with_output()).await {
        Ok(result) => {
            let output = result.context("failed to collect `rg` output")?;
            Ok(Output {
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                code: output.status.code(),
            })
        }
        Err(_) => Ok(Output {
            stdout: String::new(),
            stderr: String::new(),
            code: None,
        }),
    }
}
