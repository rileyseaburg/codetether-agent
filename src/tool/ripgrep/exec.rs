//! Spawn ripgrep and collect its output under a wall-clock budget.

use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;

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
    let policy = crate::tool::sandbox::SandboxPolicy {
        allow_exec: true,
        allow_network: false,
        ..Default::default()
    };
    let child = crate::tool::sandbox::sandbox_spawn_piped::spawn("rg", flags, &policy, cwd)
        .await
        .context("failed to spawn sandboxed `rg`; is ripgrep installed and on PATH?")?;
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
