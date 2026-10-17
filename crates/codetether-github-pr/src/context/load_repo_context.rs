//! GitHub repository load repo context.
use super::*;

/// Load repository metadata and the environment/trailer signature context.
///
/// # Arguments
/// * `cwd` - Optional directory for Git commands.
/// * `head_override` - Non-empty branch name to use instead of the current branch.
/// # Returns
/// GitHub repository identity and signature metadata for PR creation.
/// # Errors
/// Returns an error for failed Git discovery, unsupported remotes, or a missing workspace ID.
/// # Examples
/// ```no_run
/// let context = codetether_github_pr::context::load_repo_context(None, None)?;
/// assert!(!context.workspace_id.is_empty());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn load_repo_context(cwd: Option<&Path>, head_override: Option<&str>) -> Result<RepoContext> {
    let repo_root = PathBuf::from(git_stdout(cwd, ["rev-parse", "--show-toplevel"])?);
    let remote_url = git_stdout(Some(&repo_root), ["remote", "get-url", "origin"])?;
    let (owner, repo) = parse_github_remote(&remote_url)?;
    let workspace_id = git_config(&repo_root, "codetether.workspaceId")?
        .ok_or_else(|| anyhow!("Missing codetether.workspaceId in local git config"))?;
    let head_branch = match head_override {
        Some(head) if !head.trim().is_empty() => head.trim().to_string(),
        _ => git_stdout(Some(&repo_root), ["branch", "--show-current"])?,
    };
    let head_sha = git_stdout(Some(&repo_root), ["rev-parse", "HEAD"])?;
    let github_installation_id = git_config(&repo_root, "codetether.githubInstallationId")?;
    let github_app_id = git_config(&repo_root, "codetether.githubAppId")?;
    let signature = SignatureContext {
        agent_identity_id: env_or_trailer(
            "CODETETHER_AGENT_IDENTITY_ID",
            &repo_root,
            "CodeTether-Agent-Identity",
        ),
        model: std::env::var("CODETETHER_CURRENT_MODEL").ok(),
        tenant_id: env_or_trailer("CODETETHER_TENANT_ID", &repo_root, "CodeTether-Tenant-ID"),
        worker_id: std::env::var("CODETETHER_WORKER_ID").ok(),
        run_id: std::env::var("CODETETHER_RUN_ID").ok(),
        attempt_id: std::env::var("CODETETHER_ATTEMPT_ID").ok(),
        session_id: std::env::var("CODETETHER_SESSION_ID").ok(),
        provenance_signature: std::env::var("CODETETHER_SIGNATURE").ok(),
        github_installation_id: github_installation_id.clone(),
        github_app_id: github_app_id.clone(),
        head_sha: head_sha.clone(),
        workspace_id: workspace_id.clone(),
    };
    Ok(RepoContext {
        repo_root,
        owner,
        repo,
        head_branch,
        head_sha,
        workspace_id,
        github_installation_id,
        github_app_id,
        signature,
    })
}
