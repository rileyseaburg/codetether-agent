//! Open a generated `.code-workspace` file in VS Code.

use super::WorktreeManager;
use anyhow::{Context, Result, anyhow};
use std::path::Path;

impl WorktreeManager {
    /// Open a multi-root `.code-workspace` file in VS Code.
    ///
    /// Runs `code <workspace>`, using `$CODETETHER_VSCODE_BIN` when set and
    /// falling back to `code` on `PATH`.
    ///
    /// # Arguments
    ///
    /// * `workspace` — Path to the `.code-workspace` file to open.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the `code` binary cannot be launched or exits
    /// non-zero (for example, when VS Code is not installed).
    pub async fn open_workspace_in_vscode(&self, workspace: &Path) -> Result<()> {
        let bin = std::env::var("CODETETHER_VSCODE_BIN").unwrap_or_else(|_| "code".to_string());
        let path = workspace.to_string_lossy().to_string();
        let output = tokio::process::Command::new(&bin)
            .arg(&path)
            .output()
            .await
            .with_context(|| format!("Failed to launch '{bin}' for workspace '{path}'"))?;
        if !output.status.success() {
            return Err(anyhow!(
                "'{bin}' exited with failure opening '{path}': {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        tracing::info!(workspace = %path, "Opened workspace in VS Code");
        Ok(())
    }
}
