use super::{WorktreeInfo, WorktreeManager, discover_parse::parse_worktree_list};
use anyhow::{Context, Result};

impl WorktreeManager {
    /// Get information about a managed worktree.
    #[allow(dead_code)]
    pub async fn get(&self, name: &str) -> Option<WorktreeInfo> {
        self.list().await.into_iter().find(|info| info.name == name)
    }

    /// List tracked and Git-discovered CodeTether worktrees.
    pub async fn list(&self) -> Vec<WorktreeInfo> {
        let mut infos = self.worktrees.lock().await.clone();
        if let Ok(discovered) = self.discover_worktrees().await {
            for info in discovered {
                push_unique(&mut infos, info);
            }
        }
        infos
    }

    pub(crate) async fn discover_worktrees(&self) -> Result<Vec<WorktreeInfo>> {
        let output = tokio::process::Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git worktree list --porcelain")?;
        if !output.status.success() {
            return Ok(Vec::new());
        }
        Ok(parse_worktree_list(
            &String::from_utf8_lossy(&output.stdout),
            &self.base_dir,
        ))
    }
}

fn push_unique(infos: &mut Vec<WorktreeInfo>, info: WorktreeInfo) {
    if infos
        .iter()
        .any(|existing| existing.name == info.name || existing.path == info.path)
    {
        return;
    }
    infos.push(info);
}
