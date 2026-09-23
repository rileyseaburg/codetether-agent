//! Refuse worker commits and pushes that would skip repository hooks.
//!
//! A repository that declares hooks (`.pre-commit-config.yaml` or
//! `.githooks/`) must have them installed in the active hooks directory.
//! Otherwise the worker fails closed instead of committing unverified work.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

use super::git_commit_push_ops as ops;

/// Hook names each declaration requires in the active hooks directory.
fn required_hooks(repo_path: &Path) -> Vec<&'static str> {
    let mut hooks = Vec::new();
    if repo_path.join(".pre-commit-config.yaml").is_file() {
        hooks.push("pre-commit");
        if declares_post_commit(repo_path) {
            hooks.push("post-commit");
        }
    }
    if repo_path.join(".githooks/pre-push").is_file() {
        hooks.push("pre-push");
    }
    hooks
}

fn declares_post_commit(repo_path: &Path) -> bool {
    std::fs::read_to_string(repo_path.join(".pre-commit-config.yaml"))
        .map(|text| text.contains("post-commit"))
        .unwrap_or(true)
}

async fn hooks_dir(repo_path: &Path) -> Result<PathBuf> {
    let path = ops::git(
        repo_path,
        &["rev-parse", "--path-format=absolute", "--git-path", "hooks"],
    )
    .await?;
    Ok(PathBuf::from(path.trim()))
}

fn executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        path.is_file()
    }
}

/// Fail unless every hook the repository declares is installed and runnable.
pub(super) async fn require_installed(repo_path: &Path) -> Result<()> {
    let required = required_hooks(repo_path);
    if required.is_empty() {
        return Ok(());
    }
    let dir = hooks_dir(repo_path).await?;
    let missing: Vec<&str> = required
        .into_iter()
        .filter(|name| !executable(&dir.join(name)))
        .collect();
    if !missing.is_empty() {
        bail!(
            "repository hooks are not installed ({}); refusing to commit or \
             push without them",
            missing.join(", ")
        );
    }
    Ok(())
}

#[cfg(test)]
#[path = "git_hook_gate_tests.rs"]
mod tests;
