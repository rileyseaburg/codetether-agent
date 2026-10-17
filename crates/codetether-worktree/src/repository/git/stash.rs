//! Repository Git stash operation.
use super::*;

pub(super) async fn stash_dirty_clone_state(repo_path: &Path) -> Result<()> {
    if run_git_command_at(Some(repo_path), vec!["status".into(), "--porcelain".into()])
        .await?
        .trim()
        .is_empty()
    {
        return Ok(());
    }
    let message = format!(
        "codetether auto-save before workspace refresh {}",
        chrono::Utc::now().to_rfc3339()
    );
    run_git_command_at(
        Some(repo_path),
        vec![
            "stash".into(),
            "push".into(),
            "--include-untracked".into(),
            "-m".into(),
            message,
        ],
    )
    .await
    .map(|_| ())
}
