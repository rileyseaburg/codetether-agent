//! Select the existing branch PR, or create one when absent.
use super::*;
/// Create or update a PR using an authenticated client and discovered context.
///
/// # Arguments
/// * `client` - Client configured with the caller's GitHub credential.
/// * `context` - Repository and head-branch identity.
/// * `spec` - Title, body, base branch, and creation draft setting.
/// # Returns
/// The created or updated PR. Existing PRs retain the original title/body-only update policy.
/// # Errors
/// Propagates request, unsuccessful HTTP status, and response decoding errors.
/// # Examples
/// ```no_run
/// # async fn example(client: &reqwest::Client, context: &codetether_github_pr::context::RepoContext) -> anyhow::Result<()> {
/// let spec = codetether_github_pr::http::PullRequestSpec {
///     title: "change", base: "main", body: "description", draft: true,
/// };
/// let pr = codetether_github_pr::http::create_or_update(client, context, &spec).await?;
/// assert!(!pr.html_url.is_empty());
/// # Ok(()) }
/// ```
pub async fn create_or_update(
    client: &Client,
    context: &RepoContext,
    spec: &PullRequestSpec<'_>,
) -> Result<PullRequestResponse> {
    let existing = find_existing(client, context).await?;
    match existing {
        Some(number) => update_pr(client, context, number, spec.title, spec.body).await,
        None => create_pr(client, context, spec, spec.body).await,
    }
}
