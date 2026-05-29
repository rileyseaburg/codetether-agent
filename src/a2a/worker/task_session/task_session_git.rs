use anyhow::Result;

use crate::session::Session;

use super::super::{TaskContext, configure_repo_git_auth, install_commit_msg_hook};

pub(super) async fn prepare_git(
    task_id: &str,
    session: &mut Session,
    context: &TaskContext,
) -> Result<()> {
    if let (Some(directory), Some(workspace_id)) = (
        session.metadata.directory.as_deref(),
        context.workspace_id.as_deref(),
    ) && directory.join(".git").exists()
        && let Err(error) = configure_repo_git_auth(directory, workspace_id)
    {
        tracing::warn!(task_id, error = %error, "Failed to configure Git credential helper");
    }
    if let Some(directory) = session.metadata.directory.as_deref()
        && let Err(error) = install_commit_msg_hook(directory)
    {
        tracing::warn!(task_id, error = %error, "Failed to install commit-msg hook");
    }
    super::super::git_branch::ensure_metadata_checked_out(
        session.metadata.directory.as_deref(),
        super::super::metadata_str(&context.metadata, &["branch_name", "git_branch", "pr_head"]),
    )
    .await
}
