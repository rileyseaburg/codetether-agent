//! Git worktree management for subagent isolation
//!
//! Each sub-agent gets its own git worktree to work in isolation,
//! preventing file conflicts and enabling parallel execution.

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Information about a created worktree
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WorktreeInfo {
    /// Unique identifier for this worktree
    pub id: String,
    /// Path to the worktree directory
    pub path: PathBuf,
    /// Branch name for this worktree
    pub branch: String,
    /// The original repository path
    pub repo_path: PathBuf,
    /// Parent branch this was created from
    pub parent_branch: String,
}

/// Manages git worktrees for sub-agent isolation
pub struct WorktreeManager {
    /// Base directory for worktrees
    base_dir: PathBuf,
    /// The main repository path
    repo_path: PathBuf,
}

impl WorktreeManager {
    /// Create a new worktree manager
    pub fn new(repo_path: impl AsRef<Path>) -> Result<Self> {
        let repo_path = repo_path.as_ref().to_path_buf();
        
        // Default worktree base: ~/.local/share/codetether/worktrees/{repo-name}
        let repo_name = repo_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        let base_dir = ProjectDirs::from("com", "codetether", "codetether-agent")
            .map(|p| p.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("/tmp/.codetether"))
            .join("worktrees")
            .join(&repo_name);
        
        std::fs::create_dir_all(&base_dir)?;
        
        Ok(Self { base_dir, repo_path })
    }
    
    /// Create a new worktree for a sub-agent
    pub fn create(&self, task_slug: &str) -> Result<WorktreeInfo> {
        let id = format!("{}-{}", task_slug, &Uuid::new_v4().to_string()[..8]);
        let branch = format!("codetether/subagent-{}", id);
        let worktree_path = self.base_dir.join(&id);
        
        // Get current branch
        let parent_branch = self.current_branch()?;
        
        info!(
            worktree_id = %id,
            branch = %branch,
            path = %worktree_path.display(),
            parent_branch = %parent_branch,
            "Creating worktree"
        );
        
        // Create the worktree with a new branch
        let output = Command::new("git")
            .args([
                "worktree",
                "add",
                "-b",
                &branch,
                worktree_path.to_str().unwrap(),
                "HEAD",
            ])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to run git worktree add")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("git worktree add failed: {}", stderr));
        }
        
        // Inject workspace stub to isolate from parent workspace
        // This prevents "current package believes it's in a workspace when it's not" errors
        self.inject_workspace_stub(&worktree_path)?;
        
        debug!(
            worktree_id = %id,
            "Worktree created successfully"
        );
        
        Ok(WorktreeInfo {
            id,
            path: worktree_path,
            branch,
            repo_path: self.repo_path.clone(),
            parent_branch,
        })
    }
    
    /// Get the current branch name
    fn current_branch(&self) -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to get current branch")?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    /// Inject a workspace stub into the worktree's Cargo.toml
    /// 
    /// This makes the worktree a standalone workspace root, preventing Cargo
    /// from walking upward to find a parent workspace (which would fail since
    /// the worktree is in a different location than the original repo).
    fn inject_workspace_stub(&self, worktree_path: &Path) -> Result<()> {
        let cargo_toml = worktree_path.join("Cargo.toml");
        
        if !cargo_toml.exists() {
            debug!("No Cargo.toml in worktree, skipping workspace stub");
            return Ok(());
        }
        
        let content = std::fs::read_to_string(&cargo_toml)
            .context("Failed to read Cargo.toml")?;
        
        // Check if already has [workspace] section
        if content.contains("[workspace]") {
            debug!("Cargo.toml already has [workspace], skipping stub");
            return Ok(());
        }
        
        // Prepend [workspace] to make this a standalone workspace root
        let new_content = format!("[workspace]\n\n{}", content);
        
        std::fs::write(&cargo_toml, new_content)
            .context("Failed to write workspace stub to Cargo.toml")?;
        
        info!(
            cargo_toml = %cargo_toml.display(),
            "Injected [workspace] stub for hermetic isolation"
        );
        
        Ok(())
    }
    
    /// Merge a worktree's changes back to the parent branch
    pub fn merge(&self, worktree: &WorktreeInfo) -> Result<MergeResult> {
        info!(
            worktree_id = %worktree.id,
            branch = %worktree.branch,
            target = %worktree.parent_branch,
            "Merging worktree changes"
        );
        
        // First, get diff stats before merge
        let diff_output = Command::new("git")
            .args(["diff", "--stat", &format!("{}..{}", worktree.parent_branch, worktree.branch)])
            .current_dir(&self.repo_path)
            .output()?;
        
        let diff_stat = String::from_utf8_lossy(&diff_output.stdout).to_string();
        
        // Count changed files
        let files_changed = diff_stat.lines()
            .filter(|l| l.contains('|'))
            .count();
        
        // Attempt the merge
        let merge_output = Command::new("git")
            .args(["merge", "--no-ff", "-m", 
                   &format!("Merge subagent worktree: {}", worktree.id),
                   &worktree.branch])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to run git merge")?;
        
        if merge_output.status.success() {
            info!(
                worktree_id = %worktree.id,
                files_changed = files_changed,
                "Merge successful"
            );
            
            Ok(MergeResult {
                success: true,
                conflicts: Vec::new(),
                files_changed,
                summary: format!("Merged {} files from subagent {}", files_changed, worktree.id),
            })
        } else {
            let stderr = String::from_utf8_lossy(&merge_output.stderr).to_string();
            
            // Check for conflicts
            let conflicts: Vec<String> = if stderr.contains("CONFLICT") {
                // Get list of conflicted files
                let status = Command::new("git")
                    .args(["diff", "--name-only", "--diff-filter=U"])
                    .current_dir(&self.repo_path)
                    .output()?;
                
                String::from_utf8_lossy(&status.stdout)
                    .lines()
                    .map(|s| s.to_string())
                    .collect()
            } else {
                Vec::new()
            };
            
            warn!(
                worktree_id = %worktree.id,
                conflicts = ?conflicts,
                "Merge had conflicts"
            );
            
            // Abort the failed merge
            let _ = Command::new("git")
                .args(["merge", "--abort"])
                .current_dir(&self.repo_path)
                .output();
            
            Ok(MergeResult {
                success: false,
                conflicts,
                files_changed: 0,
                summary: format!("Merge failed: {}", stderr),
            })
        }
    }
    
    /// Clean up a worktree after use
    pub fn cleanup(&self, worktree: &WorktreeInfo) -> Result<()> {
        info!(
            worktree_id = %worktree.id,
            path = %worktree.path.display(),
            "Cleaning up worktree"
        );
        
        // Remove the worktree
        let output = Command::new("git")
            .args(["worktree", "remove", "--force", worktree.path.to_str().unwrap()])
            .current_dir(&self.repo_path)
            .output();
        
        if let Err(e) = output {
            warn!(error = %e, "Failed to remove worktree via git");
            // Fallback: just delete the directory
            let _ = std::fs::remove_dir_all(&worktree.path);
        }
        
        // Delete the branch
        let _ = Command::new("git")
            .args(["branch", "-D", &worktree.branch])
            .current_dir(&self.repo_path)
            .output();
        
        debug!(
            worktree_id = %worktree.id,
            "Worktree cleanup complete"
        );
        
        Ok(())
    }
    
    /// List all active worktrees
    #[allow(dead_code)]
    pub fn list(&self) -> Result<Vec<WorktreeInfo>> {
        let output = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to list worktrees")?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut worktrees = Vec::new();
        
        let mut current_path: Option<PathBuf> = None;
        let mut current_branch: Option<String> = None;
        
        for line in stdout.lines() {
            if let Some(path) = line.strip_prefix("worktree ") {
                current_path = Some(PathBuf::from(path));
            } else if let Some(branch) = line.strip_prefix("branch refs/heads/") {
                current_branch = Some(branch.to_string());
            } else if line.is_empty() {
                if let (Some(path), Some(branch)) = (current_path.take(), current_branch.take()) {
                    // Only include our subagent worktrees
                    if branch.starts_with("codetether/subagent-") {
                        let id = branch
                            .strip_prefix("codetether/subagent-")
                            .unwrap_or(&branch)
                            .to_string();
                        
                        worktrees.push(WorktreeInfo {
                            id,
                            path,
                            branch,
                            repo_path: self.repo_path.clone(),
                            parent_branch: String::new(), // Unknown for listed worktrees
                        });
                    }
                }
            }
        }
        
        Ok(worktrees)
    }
    
    /// Clean up all orphaned worktrees
    #[allow(dead_code)]
    pub fn cleanup_all(&self) -> Result<usize> {
        let worktrees = self.list()?;
        let count = worktrees.len();
        
        for wt in worktrees {
            if let Err(e) = self.cleanup(&wt) {
                warn!(worktree_id = %wt.id, error = %e, "Failed to cleanup worktree");
            }
        }
        
        // Also prune any stale worktree references
        let _ = Command::new("git")
            .args(["worktree", "prune"])
            .current_dir(&self.repo_path)
            .output();
        
        Ok(count)
    }
}

/// Result of merging a worktree back
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub success: bool,
    pub conflicts: Vec<String>,
    pub files_changed: usize,
    pub summary: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    fn setup_test_repo() -> Result<(TempDir, PathBuf)> {
        let temp = TempDir::new()?;
        let repo_path = temp.path().to_path_buf();
        
        // Initialize git repo
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()?;
        
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(&repo_path)
            .output()?;
        
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(&repo_path)
            .output()?;
        
        // Create initial commit
        std::fs::write(repo_path.join("README.md"), "# Test")?;
        Command::new("git")
            .args(["add", "."])
            .current_dir(&repo_path)
            .output()?;
        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&repo_path)
            .output()?;
        
        Ok((temp, repo_path))
    }
    
    #[test]
    fn test_create_worktree() -> Result<()> {
        let (_temp, repo_path) = setup_test_repo()?;
        let manager = WorktreeManager::new(&repo_path)?;
        
        let wt = manager.create("test-task")?;
        
        assert!(wt.path.exists());
        assert!(wt.branch.starts_with("codetether/subagent-test-task-"));
        
        // Cleanup
        manager.cleanup(&wt)?;
        assert!(!wt.path.exists());
        
        Ok(())
    }
}
