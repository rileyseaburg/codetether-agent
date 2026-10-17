//! GitHub PR update pr.
use super::*;

pub(super) async fn update_pr(
    client: &Client,
    context: &RepoContext,
    number: u64,
    title: &str,
    body: &str,
) -> Result<PullRequestResponse> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        context.owner, context.repo, number
    );
    client
        .patch(url)
        .json(&UpdatePullRequestRequest { title, body })
        .send()
        .await?
        .error_for_status()?
        .json::<PullRequestResponse>()
        .await
        .map_err(Into::into)
}
