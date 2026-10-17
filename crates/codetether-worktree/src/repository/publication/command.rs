//! Repository publication command operation.
use super::*;

pub(super) async fn git(repo_path: &Path, args: &[&str]) -> Result<String> {
    run_git_command_at(
        Some(repo_path),
        args.iter().map(|arg| arg.to_string()).collect(),
    )
    .await
}
