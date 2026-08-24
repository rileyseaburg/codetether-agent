use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn repo_root(path: &Path) -> Result<Option<PathBuf>> {
    let args = vec!["rev-parse".into(), "--show-toplevel".into()];
    let output = crate::tool::git::process::output_blocking(path, &args, &[], false)
        .context("Failed to resolve git repo root")?;
    if !output.status.success() {
        return Ok(None);
    }
    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok((!root.is_empty()).then(|| PathBuf::from(root)))
}

pub fn commit_editmsg_path(repo_path: &Path) -> Result<PathBuf> {
    let args = vec![
        "rev-parse".into(),
        "--git-path".into(),
        "COMMIT_EDITMSG".into(),
    ];
    let output = crate::tool::git::process::output_blocking(repo_path, &args, &[], false)
        .context("Failed to resolve COMMIT_EDITMSG path")?;
    if !output.status.success() {
        anyhow::bail!(
            "git rev-parse failed while resolving COMMIT_EDITMSG: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    absolute_path(repo_path, String::from_utf8_lossy(&output.stdout).trim())
}

fn absolute_path(repo_path: &Path, path: &str) -> Result<PathBuf> {
    if path.is_empty() {
        anyhow::bail!("git returned an empty COMMIT_EDITMSG path");
    }
    let path = PathBuf::from(path);
    Ok(if path.is_absolute() {
        path
    } else {
        repo_path.join(path)
    })
}
