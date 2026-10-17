//! Repository publication sequence and progress notifications.
use super::*;

/// Commit and push task changes after enforcing repository hook policy.
///
/// # Arguments
/// * `repo_path` - Repository to publish.
/// * `task_id` - Task identifier included in the commit and attribution.
/// * `provenance` - Optional existing execution attribution.
/// * `checkpoint` - Called at staging, commit creation, and push transitions.
/// # Returns
/// A commit/push summary, or `None` when no repository changes exist.
/// # Errors
/// Returns hook validation, Git staging, commit, or push errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_worktree::repository::publication::run;
/// let summary = run(std::path::Path::new("repo"), "task-1", None, |_| {}).await?;
/// # Ok(()) }
/// ```
pub async fn run(
    repo_path: &Path,
    task_id: &str,
    provenance: Option<&codetether_provenance::ExecutionProvenance>,
    mut checkpoint: impl FnMut(PublicationCheckpoint),
) -> Result<Option<String>> {
    if !repo_path.join(".git").exists() || status(repo_path).await?.trim().is_empty() {
        return Ok(None);
    }
    // Never create a commit or push that skipped the repository's own hooks.
    require_installed(repo_path).await?;
    checkpoint(PublicationCheckpoint::Staging);
    git(repo_path, &["add", "--all"]).await?;
    if status(repo_path).await?.trim().is_empty() {
        return Ok(None);
    }
    commit(repo_path, task_id, provenance).await?;
    checkpoint(PublicationCheckpoint::Created);
    let commit_sha = git(repo_path, &["rev-parse", "HEAD"])
        .await
        .unwrap_or_default();
    let branch = git(repo_path, &["rev-parse", "--abbrev-ref", "HEAD"])
        .await
        .unwrap_or_else(|_| "HEAD".to_string());
    checkpoint(PublicationCheckpoint::Pushing);
    push(repo_path, branch.trim()).await?;
    checkpoint(PublicationCheckpoint::Pushed);
    Ok(Some(format!(
        "Committed and pushed task changes: {} on {}",
        commit_sha.trim(),
        branch.trim()
    )))
}
