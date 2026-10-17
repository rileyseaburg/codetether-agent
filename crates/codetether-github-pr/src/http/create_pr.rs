//! GitHub PR create pr.
use super::*;

pub(super) async fn create_pr(
    client: &Client,
    context: &RepoContext,
    args: &PullRequestSpec<'_>,
    body: &str,
) -> Result<PullRequestResponse> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        context.owner, context.repo
    );
    client
        .post(url)
        .json(&CreatePullRequestRequest {
            title: args.title,
            head: &context.head_branch,
            base: args.base,
            body,
            draft: args.draft,
        })
        .send()
        .await?
        .error_for_status()?
        .json::<PullRequestResponse>()
        .await
        .map_err(Into::into)
}
