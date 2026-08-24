use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub(super) struct IntegrationDir {
    repo: PathBuf,
    path: PathBuf,
}

impl IntegrationDir {
    pub(super) fn create(repo: &Path, base: &Path, name: &str) -> Result<Self> {
        let suffix = uuid::Uuid::new_v4().simple();
        let path = base.join(format!("integrate-{name}-{suffix}"));
        let args = vec![
            "worktree".into(),
            "add".into(),
            "--detach".into(),
            "--quiet".into(),
            path.display().to_string(),
            "HEAD".into(),
        ];
        let output = crate::tool::git::process::output_blocking(repo, &args, &[], true)
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
        let args = vec![
            "worktree".into(),
            "remove".into(),
            "--force".into(),
            self.path.display().to_string(),
        ];
        let _ = crate::tool::git::process::output_blocking(&self.repo, &args, &[], true);
    }
}
