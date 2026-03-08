//! Core WorktreeManager implementation
use crate::worktree::{
    types::{WorktreeInfo, MergeResult},
    helpers::{default_base_dir_for_repo, validate_worktree_name},
    integrity::IntegrityChecker,
    merge::MergeManager,
    cleanup::CleanupManager,
};
use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;

pub struct WorktreeManager {
    base_dir: PathBuf,
    repo_path: PathBuf,
    worktrees: Mutex<Vec<WorktreeInfo>>,
    integrity_checked: Mutex<bool>,
}

impl WorktreeManager {
    pub fn new(repo_path: impl Into<PathBuf>) -> Self {
        let repo_path = repo_path.into();
        let repo_path = repo_path.canonicalize().unwrap_or(repo_path);
        Self {
            base_dir: default_base_dir_for_repo(&repo_path),
            repo_path,
            worktrees: Mutex::new(Vec::new()),
            integrity_checked: Mutex::new(false),
        }
    }

    pub fn with_repo(base_dir: impl Into<PathBuf>, repo_path: impl Into<PathBuf>) -> Self {
        let repo_path = repo_path.into();
        let repo_path = repo_path.canonicalize().unwrap_or(repo_path);
        Self {
            base_dir: base_dir.into(),
            repo_path,
            worktrees: Mutex::new(Vec::new()),
            integrity_checked: Mutex::new(false),
        }
    }

    pub fn inject_workspace_stub(&self, worktree_path: &Path) -> Result<()> {
        let cargo_toml = worktree_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&cargo_toml)?;
        if content
            .lines()
            .any(|line| line.trim() == "[workspace]")
        {
            return Ok(());
        }

        std::fs::write(cargo_toml, format!("[workspace]\n\n{content}"))?;
        Ok(())
    }

    pub async fn create(&self, name: &str) -> Result<WorktreeInfo> {
        self.check_integrity_once().await?;
        validate_worktree_name(name)?;
        let worktree_path = self.base_dir.join(name);
        let branch = format!("codetether/{}", name);

        tokio::fs::create_dir_all(&self.base_dir).await?;

        let output = tokio::process::Command::new("git")
            .args(["worktree", "add", "-b", &branch])
            .arg(&worktree_path)
            .current_dir(&self.repo_path)
            .output().await?;

        let status = if output.status.success() {
            output
        } else {
            tokio::process::Command::new("git")
                .args(["worktree", "add"])
                .arg(&worktree_path)
                .arg(&branch)
                .current_dir(&self.repo_path)
                .output().await?
        };

        if !status.status.success() {
            return Err(anyhow!("Failed to create worktree: {}",
                String::from_utf8_lossy(&status.stderr)));
        }

        let info = WorktreeInfo { name: name.to_string(), path: worktree_path.clone(),
            branch, active: true };
        self.worktrees.lock().await.push(info.clone());
        tracing::info!(worktree = %name, path = %worktree_path.display(), "Created worktree");
        Ok(info)
    }

    pub async fn list(&self) -> Vec<WorktreeInfo> {
        self.worktrees.lock().await.clone()
    }

    pub async fn cleanup(&self, name: &str) -> Result<()> {
        let mut wts = self.worktrees.lock().await;
        if let Some(pos) = wts.iter().position(|w| w.name == name) {
            let path = wts[pos].path.clone();
            CleanupManager::remove_worktree(&self.repo_path, &path).await?;
            wts.remove(pos);
        }
        Ok(())
    }

    pub async fn merge(&self, name: &str) -> Result<MergeResult> {
        let wts = self.worktrees.lock().await;
        let branch = wts.iter().find(|w| w.name == name)
            .ok_or_else(|| anyhow!("Worktree not found: {}", name))?
            .branch.clone();
        drop(wts);
        MergeManager::merge(&self.repo_path, &branch).await
    }

    pub async fn complete_merge(&self, _name: &str, msg: &str) -> Result<MergeResult> {
        MergeManager::complete_merge(&self.repo_path, msg).await
    }

    pub async fn abort_merge(&self, _name: &str) -> Result<()> {
        MergeManager::abort_merge(&self.repo_path).await
    }

    pub async fn cleanup_all(&self) -> Result<usize> {
        let wts = self.worktrees.lock().await.clone();
        let count = CleanupManager::cleanup_all(&self.repo_path, &wts).await?;
        self.worktrees.lock().await.clear();
        Ok(count)
    }

    async fn check_integrity_once(&self) -> Result<()> {
        let mut checked = self.integrity_checked.lock().await;
        if *checked { return Ok(()); }
        IntegrityChecker::ensure_integrity(&self.repo_path).await?;
        *checked = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn with_repo_uses_explicit_base_dir() {
        let repo = Path::new("/tmp/example-repo");
        let manager = WorktreeManager::with_repo("/tmp/worktrees", repo);

        assert_eq!(manager.repo_path, repo);
        assert_eq!(manager.base_dir, Path::new("/tmp/worktrees"));
    }

    #[test]
    fn inject_workspace_stub_prepends_workspace_header_once() {
        let temp = tempdir().expect("tempdir");
        let cargo_toml = temp.path().join("Cargo.toml");
        std::fs::write(&cargo_toml, "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n")
            .expect("write cargo");

        let manager = WorktreeManager::new(temp.path());
        manager
            .inject_workspace_stub(temp.path())
            .expect("inject stub");
        manager
            .inject_workspace_stub(temp.path())
            .expect("inject stub twice");

        let updated = std::fs::read_to_string(&cargo_toml).expect("read cargo");
        assert!(updated.starts_with("[workspace]\n\n[package]\n"));
        assert_eq!(updated.matches("[workspace]").count(), 1);
    }
}
