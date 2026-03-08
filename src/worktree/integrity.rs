//! Repository integrity checks and auto-repair
use crate::worktree::helpers::{summarize_git_output, combined_output};
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::process::Command;

pub struct IntegrityChecker;

impl IntegrityChecker {
    pub async fn run_fsck(repo_path: &Path) -> Result<std::process::Output> {
        Command::new("git")
            .args(["fsck", "--full", "--no-dangling"])
            .current_dir(repo_path)
            .output()
            .await
            .map_err(|e| anyhow!("git fsck failed: {}", e))
    }

    pub async fn repair(repo_path: &Path) {
        let steps: &[&[&str]] = &[
            &["fetch", "--all", "--prune", "--tags"],
            &["worktree", "prune"],
            &["gc", "--prune=now"],
        ];
        for step in steps {
            let _ = Command::new("git").args(*step).current_dir(repo_path).output().await;
        }
    }

    pub fn looks_like_corruption(output: &str) -> bool {
        let lower = output.to_ascii_lowercase();
        ["missing blob", "missing tree", "missing commit", "bad object",
         "unable to read", "object file", "hash mismatch", "broken link",
         "corrupt", "invalid sha1", "fatal: loose object", "failed to parse"]
            .iter().any(|n| lower.contains(n))
    }

    pub fn integrity_error(repo_path: &Path, output: &str) -> anyhow::Error {
        let summary = summarize_git_output(output);
        anyhow!(
            "Git corruption in '{}': {}\nRecovery:\n1. Backup: git diff > /tmp/patch.patch\n\
             2. Try: git fetch --all && git fsck\n3. Reclone if needed.",
            repo_path.display(), summary
        )
    }

    pub async fn ensure_integrity(repo_path: &Path) -> Result<()> {
        let first = Self::run_fsck(repo_path).await?;
        if first.status.success() { return Ok(()); }

        let out = combined_output(&first.stdout, &first.stderr);
        if !Self::looks_like_corruption(&out) {
            return Err(anyhow!("Git preflight failed: {}", summarize_git_output(&out)));
        }

        tracing::warn!("Git corruption detected; auto-repairing");
        Self::repair(repo_path).await;

        let second = Self::run_fsck(repo_path).await?;
        if second.status.success() { return Ok(()); }

        Err(Self::integrity_error(repo_path, &combined_output(&second.stdout, &second.stderr)))
    }
}
