//! Repository Git checkout operation.
use super::*;

pub(super) async fn ensure_checked_out(repo_path: &Path, branch: &str) -> Result<()> {
    if branch.trim().is_empty() || !repo_path.join(".git").exists() {
        return Ok(());
    }
    if current_branch(repo_path).await?.trim() == branch {
        return Ok(());
    }
    run_git_command_at(
        Some(repo_path),
        vec!["checkout".into(), "-B".into(), branch.to_string()],
    )
    .await
    .with_context(|| format!("Failed to checkout task branch {branch}"))?;
    Ok(())
}
