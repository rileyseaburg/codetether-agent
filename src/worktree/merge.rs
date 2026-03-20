//! Merge operations for worktrees
use crate::provenance::{ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance};
use crate::worktree::{helpers::validate_worktree_name, types::MergeResult};
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::process::Command;

pub struct MergeManager;

impl MergeManager {
    pub async fn merge(repo_path: &Path, branch: &str) -> Result<MergeResult> {
        validate_worktree_name(branch)?;
        let output = Command::new("git")
            .args(["merge", "--no-ff", "--no-commit", branch])
            .current_dir(repo_path)
            .output()
            .await
            .map_err(|e| anyhow!("git merge failed: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            let commit_msg = format!("Merge branch '{}' into current branch", branch);
            let provenance =
                ExecutionProvenance::for_operation("worktree", ExecutionOrigin::LocalCli);
            let commit_output = git_commit_with_provenance(repo_path, &commit_msg, Some(&provenance))
                .await
                .map_err(|e| anyhow!("commit failed: {}", e))?;
            if !commit_output.status.success() {
                return Err(anyhow!(
                    "Merge commit failed: {}",
                    String::from_utf8_lossy(&commit_output.stderr)
                ));
            }
            let files_changed = Self::count_changed_files(repo_path).await.unwrap_or(0);
            return Ok(MergeResult {
                success: true, aborted: false, conflicts: vec![],
                conflict_diffs: vec![], files_changed,
                summary: commit_msg,
            });
        }

        if stderr.contains("CONFLICT") || stdout.contains("CONFLICT") {
            let conflicts = Self::get_conflict_list(repo_path).await?;
            let diffs = Self::get_conflict_diffs(repo_path, &conflicts).await?;
            return Ok(MergeResult {
                success: false, aborted: false, conflicts, conflict_diffs: diffs,
                files_changed: 0, summary: "Merge has conflicts".to_string(),
            });
        }
        Err(anyhow!("Merge failed: {}", stderr))
    }

    pub async fn complete_merge(repo_path: &Path, commit_msg: &str) -> Result<MergeResult> {
        let merge_head = repo_path.join(".git/MERGE_HEAD");
        if !tokio::fs::try_exists(&merge_head).await.unwrap_or(false) {
            return Err(anyhow!("Not in merge state"));
        }

        let provenance = ExecutionProvenance::for_operation("worktree", ExecutionOrigin::LocalCli);
        let output = git_commit_with_provenance(repo_path, commit_msg, Some(&provenance))
            .await
            .map_err(|e| anyhow!("commit failed: {}", e))?;

        if output.status.success() {
            let files_changed = Self::count_changed_files(repo_path).await.unwrap_or(0);
            Ok(MergeResult {
                success: true, aborted: false, conflicts: vec![],
                conflict_diffs: vec![], files_changed,
                summary: format!("Merge completed: {}", commit_msg),
            })
        } else {
            Err(anyhow!("Failed to commit merge: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    pub async fn abort_merge(repo_path: &Path) -> Result<()> {
        let merge_head = repo_path.join(".git/MERGE_HEAD");
        if !tokio::fs::try_exists(&merge_head).await.unwrap_or(false) {
            return Ok(());
        }

        let output = Command::new("git").args(["merge", "--abort"])
            .current_dir(repo_path).output().await
            .map_err(|e| anyhow!("abort failed: {}", e))?;

        if output.status.success() { Ok(()) }
        else { Err(anyhow!("Abort failed: {}", String::from_utf8_lossy(&output.stderr))) }
    }

    async fn count_changed_files(repo_path: &Path) -> Result<usize> {
        let out = Command::new("git")
            .args(["diff", "--name-only", "HEAD~1", "HEAD"])
            .current_dir(repo_path).output().await?;
        Ok(String::from_utf8_lossy(&out.stdout)
            .lines().filter(|l| !l.is_empty()).count())
    }

    async fn get_conflict_list(repo_path: &Path) -> Result<Vec<String>> {
        let out = Command::new("git")
            .args(["diff", "--name-only", "--diff-filter=U"])
            .current_dir(repo_path).output().await?;
        Ok(String::from_utf8_lossy(&out.stdout).lines()
            .map(String::from).filter(|s| !s.is_empty()).collect())
    }

    async fn get_conflict_diffs(repo_path: &Path, files: &[String]) -> Result<Vec<(String, String)>> {
        let mut diffs = Vec::new();
        for file in files {
            let out = Command::new("git").args(["diff", file])
                .current_dir(repo_path).output().await?;
            diffs.push((file.clone(), String::from_utf8_lossy(&out.stdout).to_string()));
        }
        Ok(diffs)
    }
}
