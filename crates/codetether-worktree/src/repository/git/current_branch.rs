//! Repository Git current branch operation.
use super::*;

pub(super) async fn current_branch(repo_path: &Path) -> Result<String> {
    run_git_command_at(
        Some(repo_path),
        vec!["rev-parse".into(), "--abbrev-ref".into(), "HEAD".into()],
    )
    .await
}
