//! Credential configuration operation: exclude legacy worktree helper.
use super::*;

pub(super) fn exclude_legacy_worktree_helper(repo_path: &Path) -> Result<()> {
    let exclude_path = repo_git_path(repo_path, "info/exclude")?;
    let existing = std::fs::read_to_string(&exclude_path).unwrap_or_default();
    if existing
        .lines()
        .any(|line| line.trim() == ".codetether-git-credential-helper")
    {
        return Ok(());
    }
    if let Some(parent) = exclude_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create Git exclude directory {}",
                parent.display()
            )
        })?;
    }
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&exclude_path)
        .with_context(|| format!("Failed to open Git exclude file {}", exclude_path.display()))?
        .write_all(b"\n.codetether-git-credential-helper\n")
        .with_context(|| {
            format!(
                "Failed to update Git exclude file {}",
                exclude_path.display()
            )
        })
}
