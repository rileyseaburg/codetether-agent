use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub(super) async fn verify_base(repo: &Path, base: &str) -> Result<()> {
    let reference = format!("{base}^{{commit}}");
    let output = tokio::process::Command::new("git")
        .args(["rev-parse", "--verify", "--quiet", &reference])
        .current_dir(repo)
        .output()
        .await?;
    if !output.status.success() {
        bail!("cleanup base '{base}' is not a commit");
    }
    Ok(())
}

pub(super) async fn current(repo: &Path) -> Result<PathBuf> {
    let output = tokio::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(repo)
        .output()
        .await
        .context("failed to locate current worktree")?;
    if !output.status.success() {
        bail!(
            "git rev-parse failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}
