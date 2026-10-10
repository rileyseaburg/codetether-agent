use anyhow::{Context, Result, bail};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub(super) struct IntegrationDir {
    repo: PathBuf,
    path: PathBuf,
}

impl IntegrationDir {
    pub(super) fn create(repo: &Path, base: &Path, name: &str) -> Result<Self> {
        let suffix = uuid::Uuid::new_v4().simple();
        let path = base.join(format!("integrate-{name}-{suffix}"));
        let output = Command::new("git")
            .current_dir(repo)
            .args(["worktree", "add", "--detach", "--quiet"])
            .arg(&path)
            .arg("HEAD")
            .output()
            .context("failed to create integration worktree")?;
        if !output.status.success() {
            bail!(
                "failed to create integration worktree: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(Self {
            repo: repo.into(),
            path,
        })
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for IntegrationDir {
    fn drop(&mut self) {
        let _ = Command::new("git")
            .current_dir(&self.repo)
            .args(["worktree", "remove", "--force"])
            .arg(&self.path)
            .status();
    }
}
