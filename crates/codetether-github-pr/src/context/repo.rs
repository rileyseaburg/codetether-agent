//! GitHub PR repo context.
use super::*;

/// PR RepoContext data.
///
/// # Examples
/// ```
/// # fn example(signature: codetether_github_pr::context::SignatureContext) {
/// let context = codetether_github_pr::context::RepoContext {
/// repo_root: ".".into(), owner: "owner".into(), repo: "repo".into(),
/// head_branch: "branch".into(), head_sha: "sha".into(), workspace_id: "workspace".into(),
/// github_installation_id: None, github_app_id: None, signature,
/// };
/// assert_eq!(context.owner, "owner");
/// # }
/// ```
pub struct RepoContext {
    /// repo root metadata.
    pub repo_root: PathBuf,
    /// owner metadata.
    pub owner: String,
    /// repo metadata.
    pub repo: String,
    /// head branch metadata.
    pub head_branch: String,
    /// head sha metadata.
    pub head_sha: String,
    /// workspace id metadata.
    pub workspace_id: String,
    /// github installation id metadata.
    pub github_installation_id: Option<String>,
    /// github app id metadata.
    pub github_app_id: Option<String>,
    /// signature metadata.
    pub signature: SignatureContext,
}
