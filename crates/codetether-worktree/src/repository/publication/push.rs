//! Repository publication push operation.
use super::*;

pub(super) async fn push(repo_path: &Path, branch: &str) -> Result<()> {
    let refspec = super::super::push_refspec(branch)?;
    run_git_command_at(
        Some(repo_path),
        vec!["push".into(), "origin".into(), refspec],
    )
    .await?;
    Ok(())
}
