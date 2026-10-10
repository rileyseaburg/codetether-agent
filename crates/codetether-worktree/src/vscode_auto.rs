use super::{WorktreeInfo, WorktreeManager};

impl WorktreeManager {
    /// Best-effort: prompt the user and, if approved, open a freshly created
    /// worktree in VS Code.
    ///
    /// Unlike [`WorktreeManager::open_in_vscode`], this never fails the caller.
    /// It is a no-op when `CODETETHER_WORKTREE_AUTO_VSCODE` is set to `0`,
    /// `false`, or `off`. When enabled, the user is alerted and asked for
    /// permission first (see [`super::vscode_prompt::confirm_open`]); declining
    /// or a non-interactive session (swarm, Ralph, CI) skips opening. A missing
    /// `code` binary logs a warning rather than erroring, so worktree creation
    /// still succeeds.
    pub async fn auto_open_in_vscode(&self, info: &WorktreeInfo) {
        if !self.auto_open_vscode {
            return;
        }
        if !auto_open_enabled() {
            return;
        }
        if !super::vscode_prompt::confirm_open(info) {
            return;
        }
        if let Err(e) = self.open_in_vscode(info, false).await {
            tracing::warn!(worktree = %info.name, error = %e, "Auto-open in VS Code skipped");
        }
    }
}

/// Whether automatic VS Code opening is enabled (default: on).
fn auto_open_enabled() -> bool {
    match std::env::var("CODETETHER_WORKTREE_AUTO_VSCODE") {
        Ok(v) => !matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "0" | "false" | "off"
        ),
        Err(_) => true,
    }
}
