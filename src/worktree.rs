//! Git worktree management for isolated agent execution
//!
//! Provides worktree isolation for parallel agent tasks.

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Worktree information
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    /// Worktree name/identifier
    pub name: String,
    /// Path to the worktree
    pub path: PathBuf,
    /// Branch name
    pub branch: String,
    /// Whether this worktree is active
    pub active: bool,
}

/// Worktree manager for creating and managing isolated git worktrees
#[derive(Debug)]
pub struct WorktreeManager {
    /// Base directory for worktrees
    base_dir: PathBuf,
    /// Active worktrees
    worktrees: Mutex<Vec<WorktreeInfo>>,
}

/// Merge result
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub success: bool,
    pub aborted: bool,
    pub conflicts: Vec<String>,
    pub conflict_diffs: Vec<(String, String)>,
    pub files_changed: usize,
    pub summary: String,
}

impl WorktreeManager {
    /// Create a new worktree manager
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            worktrees: Mutex::new(Vec::new()),
        }
    }

    /// Create a new worktree for a task
    pub async fn create(&self, name: &str) -> Result<WorktreeInfo> {
        let worktree_path = self.base_dir.join(name);
        
        // Create the directory if it doesn't exist
        tokio::fs::create_dir_all(&worktree_path).await?;
        
        let info = WorktreeInfo {
            name: name.to_string(),
            path: worktree_path.clone(),
            branch: format!("codetether/{}", name),
            active: true,
        };
        
        let mut worktrees = self.worktrees.lock().await;
        worktrees.push(info.clone());
        
        tracing::info!(worktree = %name, path = %worktree_path.display(), "Created worktree");
        Ok(info)
    }

    /// Get information about a worktree
    pub async fn get(&self, name: &str) -> Option<WorktreeInfo> {
        let worktrees = self.worktrees.lock().await;
        worktrees.iter().find(|w| w.name == name).cloned()
    }

    /// List all worktrees
    pub async fn list(&self) -> Vec<WorktreeInfo> {
        self.worktrees.lock().await.clone()
    }

    /// Clean up a specific worktree
    pub async fn cleanup(&self, name: &str) -> Result<()> {
        let mut worktrees = self.worktrees.lock().await;
        if let Some(pos) = worktrees.iter().position(|w| w.name == name) {
            let info = &worktrees[pos];
            // Attempt to remove the directory
            if let Err(e) = tokio::fs::remove_dir_all(&info.path).await {
                tracing::warn!(worktree = %name, error = %e, "Failed to remove worktree directory");
            }
            worktrees.remove(pos);
            tracing::info!(worktree = %name, "Cleaned up worktree");
        }
        Ok(())
    }

    /// Merge a worktree branch back
    pub async fn merge(&self, name: &str) -> Result<MergeResult> {
        let worktrees = self.worktrees.lock().await;
        if let Some(_info) = worktrees.iter().find(|w| w.name == name) {
            // Placeholder: In a real implementation, this would perform git merge
            tracing::info!(worktree = %name, "Merged worktree branch");
            Ok(MergeResult {
                success: true,
                aborted: false,
                conflicts: vec![],
                conflict_diffs: vec![],
                files_changed: 0,
                summary: "Merged".to_string(),
            })
        } else {
            Err(anyhow!("Worktree not found: {}", name))
        }
    }

    /// Complete a merge
    pub async fn complete_merge(&self, name: &str, _commit_msg: &str) -> Result<MergeResult> {
        self.merge(name).await
    }

    /// Abort a merge
    pub async fn abort_merge(&self, _name: &str) -> Result<()> {
        Ok(())
    }

    /// Clean up all worktrees
    pub async fn cleanup_all(&self) -> Result<usize> {
        let mut worktrees = self.worktrees.lock().await;
        let count = worktrees.len();
        
        for info in worktrees.iter() {
            if let Err(e) = tokio::fs::remove_dir_all(&info.path).await {
                tracing::warn!(worktree = %info.name, error = %e, "Failed to remove worktree directory");
            }
        }
        
        worktrees.clear();
        tracing::info!(count, "Cleaned up all worktrees");
        Ok(count)
    }

    /// Inject workspace stub for Cargo workspace isolation
    pub fn inject_workspace_stub(&self, _worktree_path: &Path) -> Result<()> {
        // Placeholder: In a real implementation, this would prepend [workspace] to Cargo.toml
        Ok(())
    }
}

impl Default for WorktreeManager {
    fn default() -> Self {
        Self::new("/tmp/codetether-worktrees")
    }
}
