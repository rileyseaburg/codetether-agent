//! Native CLI compatibility wrapper for the shared Git credential helper.
pub async fn run_git_credential_helper(
    args: &crate::cli::GitCredentialHelperArgs,
) -> anyhow::Result<()> {
    codetether_git_credential_helper::run_git_credential_helper(args).await
}
