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

        Ok(Self {
            base_dir,
            repo_path,
        })
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

    /// Inject a `[workspace]` stub into the worktree's Cargo.toml to make it hermetically sealed
    ///
    /// This prevents Cargo from treating the worktree as part of the parent workspace,
    /// which would cause "current package believes it's in a workspace when it's not" errors.
    pub fn inject_workspace_stub(&self, worktree_path: &Path) -> Result<()> {
        let cargo_toml = worktree_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Ok(()); // No Cargo.toml, nothing to do
        }

        let content = std::fs::read_to_string(&cargo_toml).context("Failed to read Cargo.toml")?;

        // Check if already has [workspace]
        if content.contains("[workspace]") {
            return Ok(());
        }

        // Prepend [workspace] stub to make this package standalone
        let new_content = format!("[workspace]\n\n{}", content);
        std::fs::write(&cargo_toml, new_content)
            .context("Failed to write Cargo.toml with workspace stub")?;

        info!(
            cargo_toml = %cargo_toml.display(),
            "Injected [workspace] stub for hermetic isolation"
        );

        Ok(())
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

    /// Auto-commit any uncommitted changes in a worktree
    ///
    /// Sub-agents modify files via tools (write, edit, bash) but never git-commit.
    /// This stages and commits those changes so the branch actually advances
    /// before we attempt to merge it.
    fn auto_commit_worktree(&self, worktree: &WorktreeInfo) -> Result<bool> {
        // Check for uncommitted changes
        let status = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&worktree.path)
            .output()
            .context("Failed to check worktree status")?;

        let status_output = String::from_utf8_lossy(&status.stdout);
        if status_output.trim().is_empty() {
            debug!(
                worktree_id = %worktree.id,
                "No uncommitted changes in worktree"
            );
            return Ok(false);
        }

        let changed_files = status_output.lines().count();
        info!(
            worktree_id = %worktree.id,
            changed_files = changed_files,
            "Auto-committing sub-agent changes in worktree"
        );

        // Stage all changes
        let add_output = Command::new("git")
            .args(["add", "-A"])
            .current_dir(&worktree.path)
            .output()
            .context("Failed to stage worktree changes")?;

        if !add_output.status.success() {
            let stderr = String::from_utf8_lossy(&add_output.stderr);
            warn!(
                worktree_id = %worktree.id,
                error = %stderr,
                "git add -A failed in worktree"
            );
            return Err(anyhow::anyhow!("Failed to stage changes: {}", stderr));
        }

        // Commit
        let commit_msg = format!("subagent({}): automated work", worktree.id);
        let commit_output = Command::new("git")
            .args(["commit", "-m", &commit_msg])
            .current_dir(&worktree.path)
            .output()
            .context("Failed to commit worktree changes")?;

        if !commit_output.status.success() {
            let stderr = String::from_utf8_lossy(&commit_output.stderr);
            // "nothing to commit" is OK — race between status and add
            if stderr.contains("nothing to commit") {
                debug!(worktree_id = %worktree.id, "Nothing to commit after staging");
                return Ok(false);
            }
            warn!(
                worktree_id = %worktree.id,
                error = %stderr,
                "git commit failed in worktree"
            );
            return Err(anyhow::anyhow!("Failed to commit changes: {}", stderr));
        }

        info!(
            worktree_id = %worktree.id,
            changed_files = changed_files,
            "Auto-committed sub-agent changes"
        );
        Ok(true)
    }

    /// Merge a worktree's changes back to the parent branch
    pub fn merge(&self, worktree: &WorktreeInfo) -> Result<MergeResult> {
        info!(
            worktree_id = %worktree.id,
            branch = %worktree.branch,
            target = %worktree.parent_branch,
            "Merging worktree changes"
        );

        // Auto-commit any uncommitted changes the sub-agent left behind
        match self.auto_commit_worktree(worktree) {
            Ok(committed) => {
                if committed {
                    info!(worktree_id = %worktree.id, "Auto-committed sub-agent changes before merge");
                }
            }
            Err(e) => {
                warn!(
                    worktree_id = %worktree.id,
                    error = %e,
                    "Failed to auto-commit worktree changes — merge may show nothing"
                );
            }
        }

        // Check if there's already a merge in progress
        if self.is_merging() {
            warn!(
                worktree_id = %worktree.id,
                "Merge already in progress - cannot start new merge"
            );
            return Ok(MergeResult {
                success: false,
                conflicts: Vec::new(),
                conflict_diffs: Vec::new(),
                files_changed: 0,
                summary: "Merge already in progress".to_string(),
                aborted: false,
            });
        }

        // First, get diff stats before merge
        let diff_output = Command::new("git")
            .args([
                "diff",
                "--stat",
                &format!("{}..{}", worktree.parent_branch, worktree.branch),
            ])
            .current_dir(&self.repo_path)
            .output()?;

        let diff_stat = String::from_utf8_lossy(&diff_output.stdout).to_string();

        // Count changed files
        let files_changed = diff_stat.lines().filter(|l| l.contains('|')).count();

        // Log what the sub-agent actually did (trust but verify)
        if files_changed == 0 {
            warn!(
                worktree_id = %worktree.id,
                "Sub-agent made NO file changes - lazy or stuck?"
            );
        } else {
            // Get the actual diff to check for lazy patterns
            let full_diff = Command::new("git")
                .args([
                    "diff",
                    &format!("{}..{}", worktree.parent_branch, worktree.branch),
                ])
                .current_dir(&self.repo_path)
                .output()?;
            let diff_content = String::from_utf8_lossy(&full_diff.stdout);

            // Check for lazy patterns
            let lazy_indicators = [
                ("TODO", diff_content.matches("TODO").count()),
                ("FIXME", diff_content.matches("FIXME").count()),
                (
                    "unimplemented!",
                    diff_content.matches("unimplemented!").count(),
                ),
                ("todo!", diff_content.matches("todo!").count()),
                ("panic!", diff_content.matches("panic!").count()),
            ];

            let lazy_count: usize = lazy_indicators.iter().map(|(_, c)| c).sum();
            if lazy_count > 0 {
                let lazy_details: Vec<String> = lazy_indicators
                    .iter()
                    .filter(|(_, c)| *c > 0)
                    .map(|(name, c)| format!("{}:{}", name, c))
                    .collect();
                warn!(
                    worktree_id = %worktree.id,
                    lazy_markers = %lazy_details.join(", "),
                    "Sub-agent left lazy markers - review carefully!"
                );
            }

            info!(
                worktree_id = %worktree.id,
                files_changed = files_changed,
                diff_summary = %diff_stat.trim(),
                "Sub-agent changes to merge"
            );
        }

        // Attempt the merge
        let merge_output = Command::new("git")
            .args([
                "merge",
                "--no-ff",
                "-m",
                &format!("Merge subagent worktree: {}", worktree.id),
                &worktree.branch,
            ])
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
                conflict_diffs: Vec::new(),
                files_changed,
                summary: format!(
                    "Merged {} files from subagent {}",
                    files_changed, worktree.id
                ),
                aborted: false,
            })
        } else {
            let stderr = String::from_utf8_lossy(&merge_output.stderr).to_string();
            let stdout = String::from_utf8_lossy(&merge_output.stdout).to_string();

            // Check for conflicts
            let conflicts: Vec<String> =
                if stderr.contains("CONFLICT") || stdout.contains("CONFLICT") {
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

            // Better logging for different failure modes
            if stderr.contains("Already up to date") || stdout.contains("Already up to date") {
                warn!(
                    worktree_id = %worktree.id,
                    "Merge says 'Already up to date' - sub-agent may have made no commits"
                );
            } else if conflicts.is_empty() {
                // Failed but no conflicts - log what happened
                warn!(
                    worktree_id = %worktree.id,
                    stderr = %stderr.trim(),
                    stdout = %stdout.trim(),
                    "Merge failed for unknown reason (not conflicts)"
                );
            } else {
                warn!(
                    worktree_id = %worktree.id,
                    conflicts = ?conflicts,
                    "Merge had conflicts - sub-agent's changes conflict with main"
                );
            }

            // Get conflict diffs if there are actual conflicts
            let conflict_diffs: Vec<(String, String)> = if !conflicts.is_empty() {
                conflicts
                    .iter()
                    .filter_map(|file| {
                        let output = Command::new("git")
                            .args(["diff", file])
                            .current_dir(&self.repo_path)
                            .output()
                            .ok()?;
                        let diff = String::from_utf8_lossy(&output.stdout).to_string();
                        if diff.is_empty() {
                            None
                        } else {
                            Some((file.clone(), diff))
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            };

            // Only abort if no real conflicts (for non-conflict failures)
            // Keep merge state for conflict resolution
            let aborted = if conflicts.is_empty() {
                let _ = Command::new("git")
                    .args(["merge", "--abort"])
                    .current_dir(&self.repo_path)
                    .output();
                true
            } else {
                // Don't abort - leave in conflicted state for resolver
                info!(
                    worktree_id = %worktree.id,
                    num_conflicts = conflicts.len(),
                    "Leaving merge in conflicted state for resolution"
                );
                false
            };

            Ok(MergeResult {
                success: false,
                conflicts,
                conflict_diffs,
                files_changed: 0,
                summary: format!("Merge failed: {}", stderr),
                aborted,
            })
        }
    }

    /// Complete a merge after conflicts have been resolved
    pub fn complete_merge(
        &self,
        worktree: &WorktreeInfo,
        commit_message: &str,
    ) -> Result<MergeResult> {
        info!(
            worktree_id = %worktree.id,
            "Completing merge after conflict resolution"
        );

        // Stage all resolved files
        let add_output = Command::new("git")
            .args(["add", "-A"])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to stage resolved files")?;

        if !add_output.status.success() {
            let stderr = String::from_utf8_lossy(&add_output.stderr);
            warn!(error = %stderr, "Failed to stage resolved files");
        }

        // Complete the merge commit
        let commit_output = Command::new("git")
            .args(["commit", "-m", commit_message])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to complete merge commit")?;

        if commit_output.status.success() {
            // Get files changed from commit
            let stat_output = Command::new("git")
                .args(["diff", "--stat", "HEAD~1", "HEAD"])
                .current_dir(&self.repo_path)
                .output()?;
            let diff_stat = String::from_utf8_lossy(&stat_output.stdout);
            let files_changed = diff_stat.lines().filter(|l| l.contains('|')).count();

            info!(
                worktree_id = %worktree.id,
                files_changed = files_changed,
                "Merge completed after conflict resolution"
            );

            Ok(MergeResult {
                success: true,
                conflicts: Vec::new(),
                conflict_diffs: Vec::new(),
                files_changed,
                summary: format!("Merge completed after resolving conflicts"),
                aborted: false,
            })
        } else {
            let stderr = String::from_utf8_lossy(&commit_output.stderr).to_string();
            warn!(error = %stderr, "Failed to complete merge commit");

            Ok(MergeResult {
                success: false,
                conflicts: Vec::new(),
                conflict_diffs: Vec::new(),
                files_changed: 0,
                summary: format!("Failed to complete merge: {}", stderr),
                aborted: false,
            })
        }
    }

    /// Abort a merge in progress
    pub fn abort_merge(&self) -> Result<()> {
        info!("Aborting merge in progress");
        let _ = Command::new("git")
            .args(["merge", "--abort"])
            .current_dir(&self.repo_path)
            .output();
        Ok(())
    }

    /// Check if there's a merge in progress
    pub fn is_merging(&self) -> bool {
        self.repo_path.join(".git/MERGE_HEAD").exists()
    }

    /// Clean up a worktree after use
    pub fn cleanup(&self, worktree: &WorktreeInfo) -> Result<()> {
        info!(
            worktree_id = %worktree.id,
            path = %worktree.path.display(),
            "Cleaning up worktree"
        );

        // Check if there's an aborted merge and clean it up first
        if self.is_merging() {
            warn!(
                worktree_id = %worktree.id,
                "Aborted merge detected during cleanup - aborting merge state"
            );
            let _ = self.abort_merge();
        }

        // Remove the worktree
        let output = Command::new("git")
            .args([
                "worktree",
                "remove",
                "--force",
                worktree.path.to_str().unwrap(),
            ])
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

        // Clean up orphaned branches (branches that exist but worktrees don't)
        let orphaned = self.cleanup_orphaned_branches()?;

        Ok(count + orphaned)
    }

    /// Clean up orphaned subagent branches (branches with no corresponding worktree)
    pub fn cleanup_orphaned_branches(&self) -> Result<usize> {
        // List all branches matching our pattern
        let output = Command::new("git")
            .args(["branch", "--list", "codetether/subagent-*"])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to list branches")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let branches: Vec<&str> = stdout
            .lines()
            .map(|l| l.trim().trim_start_matches("* "))
            .filter(|l| !l.is_empty())
            .collect();

        // Get active worktrees
        let active_worktrees = self.list()?;
        let active_branches: std::collections::HashSet<&str> = active_worktrees
            .iter()
            .map(|wt| wt.branch.as_str())
            .collect();

        let mut deleted = 0;
        for branch in branches {
            if !active_branches.contains(branch) {
                info!(branch = %branch, "Deleting orphaned subagent branch");
                let result = Command::new("git")
                    .args(["branch", "-D", branch])
                    .current_dir(&self.repo_path)
                    .output();

                match result {
                    Ok(output) if output.status.success() => {
                        deleted += 1;
                    }
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        warn!(branch = %branch, error = %stderr, "Failed to delete orphaned branch");
                    }
                    Err(e) => {
                        warn!(branch = %branch, error = %e, "Failed to run git branch -D");
                    }
                }
            }
        }

        if deleted > 0 {
            info!(count = deleted, "Cleaned up orphaned subagent branches");
        }

        Ok(deleted)
    }
}

/// Result of merging a worktree back
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub success: bool,
    pub conflicts: Vec<String>,
    /// Diffs for conflicting files (file path -> diff content)
    pub conflict_diffs: Vec<(String, String)>,
    pub files_changed: usize,
    pub summary: String,
    /// Whether merge was aborted (false = still in conflicted state)
    pub aborted: bool,
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
