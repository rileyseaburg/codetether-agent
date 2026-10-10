use anyhow::{Context, Result, bail};
use std::{path::Path, process::Command};

pub(super) fn verify_same_repository(source: &Path, worktree: &Path) -> Result<()> {
    let source_git = git_output(source, &["rev-parse", "--git-common-dir"])?;
    let worktree_git = git_output(worktree, &["rev-parse", "--git-common-dir"])?;
    let source_git = source.join(source_git).canonicalize()?;
    let worktree_git = worktree.join(worktree_git).canonicalize()?;
    if source_git != worktree_git {
        bail!("source_repo and worktree do not belong to the same Git repository");
    }
    Ok(())
}

pub(super) fn require_ignored(worktree: &Path, relative: &Path) -> Result<()> {
    let ignored_directory = format!("{}/", relative.display());
    let status = Command::new("git")
        .current_dir(worktree)
        .args(["check-ignore", "--quiet", "--", &ignored_directory])
        .status()?;
    if !status.success() {
        bail!("refusing to link unignored path {}", relative.display());
    }
    Ok(())
}

fn git_output(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git in {}", root.display()))?;
    if !output.status.success() {
        bail!("git {} failed", args.join(" "));
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
