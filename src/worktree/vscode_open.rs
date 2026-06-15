use super::{WorktreeInfo, WorktreeManager};
use anyhow::{Context, Result, anyhow};

impl WorktreeManager {
    /// Open a worktree in VS Code.
    ///
    /// Runs `code <path>` (new window) or `code -r <path>` (reuse current
    /// window). The binary is taken from `$CODETETHER_VSCODE_BIN`, falling
    /// back to `code` on `PATH`.
    ///
    /// # Arguments
    ///
    /// * `info` — The worktree to open.
    /// * `reuse_window` — When true, reuse the current window via `-r`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the `code` binary cannot be launched or exits
    /// non-zero (for example, when VS Code is not installed).
    pub async fn open_in_vscode(&self, info: &WorktreeInfo, reuse_window: bool) -> Result<()> {
        let bin = std::env::var("CODETETHER_VSCODE_BIN").unwrap_or_else(|_| "code".to_string());
        let path = info.path.to_string_lossy().to_string();
        let mut cmd = tokio::process::Command::new(&bin);
        if reuse_window {
            cmd.arg("-r");
        }
        cmd.arg(&path);
        let output = cmd
            .output()
            .await
            .with_context(|| format!("Failed to launch '{bin}' for worktree '{}'", info.name))?;
        if !output.status.success() {
            return Err(anyhow!(
                "'{bin}' exited with failure opening '{}': {}",
                info.name,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        tracing::info!(worktree = %info.name, path = %path, reuse = reuse_window, "Opened worktree in VS Code");
        Ok(())
    }
}
