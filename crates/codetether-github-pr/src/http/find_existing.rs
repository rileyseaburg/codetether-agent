//! GitHub PR find existing.
use super::*;

pub(super) async fn find_existing(client: &Client, context: &RepoContext) -> Result<Option<u64>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls?state=open&head={}:{}",
        context.owner, context.repo, context.owner, context.head_branch
    );
    let prs = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<PullRequestResponse>>()
        .await?;
    Ok(prs.into_iter().next().map(|pr| pr.number))
}
