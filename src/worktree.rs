//! Git worktree management for isolated agent execution
//!
//! Provides worktree isolation for parallel agent tasks.

use anyhow::{anyhow, Context, Result};
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
    /// Path to the main repository
    repo_path: PathBuf,
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
            repo_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            worktrees: Mutex::new(Vec::new()),
        }
    }
    
    /// Create a worktree manager with explicit repo path
    pub fn with_repo(base_dir: impl Into<PathBuf>, repo_path: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            repo_path: repo_path.into(),
            worktrees: Mutex::new(Vec::new()),
        }
    }

    /// Create a new worktree for a task
    ///
    /// This creates an actual git worktree using `git worktree add`,
    /// creating a new branch if it doesn't exist.
    pub async fn create(&self, name: &str) -> Result<WorktreeInfo> {
        let worktree_path = self.base_dir.join(name);
        let branch_name = format!("codetether/{}", name);
        
        // Ensure base directory exists
        tokio::fs::create_dir_all(&self.base_dir).await
            .with_context(|| format!("Failed to create base directory: {}", self.base_dir.display()))?;
        
        // Run git worktree add
        let output = tokio::process::Command::new("git")
            .args(["worktree", "add", "-b", &branch_name])
            .arg(&worktree_path)
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git worktree add")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Branch might already exist, try without -b
            let output2 = tokio::process::Command::new("git")
                .args(["worktree", "add"])
                .arg(&worktree_path)
                .current_dir(&self.repo_path)
                .output()
                .await
                .context("Failed to execute git worktree add (fallback)")?;
            
            if !output2.status.success() {
                return Err(anyhow!(
                    "Failed to create git worktree '{}': {}",
                    name,
                    String::from_utf8_lossy(&output2.stderr)
                ));
            }
        }
        
        let info = WorktreeInfo {
            name: name.to_string(),
            path: worktree_path.clone(),
            branch: branch_name,
            active: true,
        };
        
        let mut worktrees = self.worktrees.lock().await;
        worktrees.push(info.clone());
        
        tracing::info!(worktree = %name, path = %worktree_path.display(), "Created git worktree");
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
            
            // Run git worktree remove
            let output = tokio::process::Command::new("git")
                .args(["worktree", "remove", "--force"])
                .arg(&info.path)
                .current_dir(&self.repo_path)
                .output()
                .await;
            
            match output {
                Ok(o) if o.status.success() => {
                    tracing::info!(worktree = %name, "Removed git worktree");
                }
                Ok(o) => {
                    tracing::warn!(
                        worktree = %name,
                        error = %String::from_utf8_lossy(&o.stderr),
                        "Git worktree remove failed, falling back to directory removal"
                    );
                    // Fallback to directory removal
                    if let Err(e) = tokio::fs::remove_dir_all(&info.path).await {
                        tracing::warn!(worktree = %name, error = %e, "Failed to remove worktree directory");
                    }
                }
                Err(e) => {
                    tracing::warn!(worktree = %name, error = %e, "Failed to execute git worktree remove");
                    // Fallback to directory removal
                    if let Err(e) = tokio::fs::remove_dir_all(&info.path).await {
                        tracing::warn!(worktree = %name, error = %e, "Failed to remove worktree directory");
                    }
                }
            }
            
            worktrees.remove(pos);
        }
        Ok(())
    }

    /// Merge a worktree branch back into the current branch
    ///
    /// This performs an actual git merge operation and handles conflicts.
    pub async fn merge(&self, name: &str) -> Result<MergeResult> {
        let worktrees = self.worktrees.lock().await;
        let info = worktrees
            .iter()
            .find(|w| w.name == name)
            .ok_or_else(|| anyhow!("Worktree not found: {}", name))?;
        
        let branch = info.branch.clone();
        drop(worktrees); // Release lock before git operations
        
        tracing::info!(worktree = %name, branch = %branch, "Starting git merge");
        
        // Run git merge
        let output = tokio::process::Command::new("git")
            .args(["merge", "--no-ff", &branch])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git merge")?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if output.status.success() {
            tracing::info!(worktree = %name, branch = %branch, "Git merge successful");
            
            // Get files changed count
            let files_changed = self.count_merge_files_changed().await.unwrap_or(0);
            
            Ok(MergeResult {
                success: true,
                aborted: false,
                conflicts: vec![],
                conflict_diffs: vec![],
                files_changed,
                summary: stdout.lines().next().unwrap_or("Merged").to_string(),
            })
        } else {
            // Check for conflicts
            if stderr.contains("CONFLICT") || stdout.contains("CONFLICT") {
                tracing::warn!(worktree = %name, "Merge has conflicts");
                
                let conflicts = self.get_conflict_list().await?;
                let conflict_diffs = self.get_conflict_diffs().await?;
                
                Ok(MergeResult {
                    success: false,
                    aborted: false,
                    conflicts,
                    conflict_diffs,
                    files_changed: 0,
                    summary: "Merge has conflicts that need resolution".to_string(),
                })
            } else {
                Err(anyhow!("Git merge failed: {}", stderr))
            }
        }
    }

    /// Complete a merge after conflicts are resolved
    ///
    /// This commits the merge after the user has resolved conflicts.
    pub async fn complete_merge(&self, name: &str, commit_msg: &str) -> Result<MergeResult> {
        let worktrees = self.worktrees.lock().await;
        let info = worktrees
            .iter()
            .find(|w| w.name == name)
            .ok_or_else(|| anyhow!("Worktree not found: {}", name))?;
        
        let branch = info.branch.clone();
        drop(worktrees);
        
        // Check if we're in a merge state
        let merge_head = self.repo_path.join(".git/MERGE_HEAD");
        let in_merge = tokio::fs::try_exists(&merge_head).await.unwrap_or(false);
        
        if !in_merge {
            return Err(anyhow!("Not in a merge state. Use merge() first."));
        }
        
        // Commit the merge
        let output = tokio::process::Command::new("git")
            .args(["commit", "-m", commit_msg])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git commit")?;
        
        if output.status.success() {
            tracing::info!(worktree = %name, branch = %branch, "Merge completed");
            
            let files_changed = self.count_merge_files_changed().await.unwrap_or(0);
            
            Ok(MergeResult {
                success: true,
                aborted: false,
                conflicts: vec![],
                conflict_diffs: vec![],
                files_changed,
                summary: format!("Merge completed: {}", commit_msg),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to complete merge: {}", stderr))
        }
    }

    /// Abort an in-progress merge
    pub async fn abort_merge(&self, _name: &str) -> Result<()> {
        // Check if we're in a merge state
        let merge_head = self.repo_path.join(".git/MERGE_HEAD");
        let in_merge = tokio::fs::try_exists(&merge_head).await.unwrap_or(false);
        
        if !in_merge {
            tracing::warn!("Not in a merge state, nothing to abort");
            return Ok(());
        }
        
        let output = tokio::process::Command::new("git")
            .args(["merge", "--abort"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git merge --abort")?;
        
        if output.status.success() {
            tracing::info!("Merge aborted");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to abort merge: {}", stderr))
        }
    }

    /// Clean up all worktrees
    pub async fn cleanup_all(&self) -> Result<usize> {
        let mut worktrees = self.worktrees.lock().await;
        let count = worktrees.len();
        
        for info in worktrees.iter() {
            // Try git worktree remove first
            let _ = tokio::process::Command::new("git")
                .args(["worktree", "remove", "--force"])
                .arg(&info.path)
                .current_dir(&self.repo_path)
                .output()
                .await;
            
            // Fallback to directory removal
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
    
    /// Get list of conflicting files during a merge
    async fn get_conflict_list(&self) -> Result<Vec<String>> {
        let output = tokio::process::Command::new("git")
            .args(["diff", "--name-only", "--diff-filter=U"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to get conflict list")?;
        
        let conflicts = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(String::from)
            .filter(|s| !s.is_empty())
            .collect();
        
        Ok(conflicts)
    }
    
    /// Get diffs for conflicting files
    async fn get_conflict_diffs(&self) -> Result<Vec<(String, String)>> {
        let conflicts = self.get_conflict_list().await?;
        let mut diffs = Vec::new();
        
        for file in conflicts {
            let output = tokio::process::Command::new("git")
                .args(["diff", &file])
                .current_dir(&self.repo_path)
                .output()
                .await;
            
            if let Ok(o) = output {
                let diff = String::from_utf8_lossy(&o.stdout).to_string();
                diffs.push((file, diff));
            }
        }
        
        Ok(diffs)
    }
    
    /// Count files changed in the last merge
    async fn count_merge_files_changed(&self) -> Result<usize> {
        let output = tokio::process::Command::new("git")
            .args(["diff", "--name-only", "HEAD~1", "HEAD"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to count changed files")?;
        
        let count = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|s| !s.is_empty())
            .count();
        
        Ok(count)
    }
}

impl Default for WorktreeManager {
    fn default() -> Self {
        Self::new("/tmp/codetether-worktrees")
    }
}
