use super::body::merge_signature;
use super::context::{RepoContext, load_repo_context};
use crate::a2a::git_credentials::{GitCredentialQuery, request_git_credentials};
use crate::cli::CreatePrArgs;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct PullRequestResult {
    pub number: u64,
    pub html_url: String,
    pub state: String,
    pub draft: bool,
    pub head_sha: String,
    pub json: bool,
}

#[derive(Debug, Deserialize)]
struct PullRequestResponse {
    number: u64,
    html_url: String,
    state: String,
    draft: bool,
}

#[derive(Debug, Serialize)]
struct CreatePullRequestRequest<'a> {
    title: &'a str,
    head: &'a str,
    base: &'a str,
    body: &'a str,
    draft: bool,
}

#[derive(Debug, Serialize)]
struct UpdatePullRequestRequest<'a> {
    title: &'a str,
    body: &'a str,
}

pub async fn create_or_update(args: CreatePrArgs) -> Result<PullRequestResult> {
    let context = load_repo_context(project_root(args.project.as_ref()), args.head.as_deref())?;
    let body = build_body(&args, &context)?;
    let client = github_client(&context).await?;
    let existing = find_existing(&client, &context).await?;
    let pr = match existing {
        Some(number) => update_pr(&client, &context, number, &args.title, &body).await?,
        None => create_pr(&client, &context, &args, &body).await?,
    };
    Ok(PullRequestResult {
        number: pr.number,
        html_url: pr.html_url,
        state: pr.state,
        draft: pr.draft,
        head_sha: context.head_sha,
        json: args.json,
    })
}

fn project_root(project: Option<&PathBuf>) -> Option<&std::path::Path> {
    project.map(PathBuf::as_path)
}

fn build_body(args: &CreatePrArgs, context: &RepoContext) -> Result<String> {
    let base = match (&args.body, &args.body_file) {
        (Some(body), _) => body.clone(),
        (None, Some(path)) => std::fs::read_to_string(path)?,
        (None, None) => String::new(),
    };
    Ok(merge_signature(&base, &context.signature))
}

async fn github_client(context: &RepoContext) -> Result<Client> {
    let server = std::env::var("CODETETHER_SERVER")
        .map_err(|_| anyhow!("CODETETHER_SERVER is required for codetether pr create"))?;
    let token = std::env::var("CODETETHER_TOKEN").ok();
    let worker_id = std::env::var("CODETETHER_WORKER_ID").ok();
    let path = format!("{}/{}.git", context.owner, context.repo);
    let credentials = request_git_credentials(
        &server,
        &token,
        worker_id.as_deref(),
        &context.workspace_id,
        "get",
        &GitCredentialQuery {
            protocol: Some("https".to_string()),
            host: Some("github.com".to_string()),
            path: Some(path),
        },
    )
    .await?
    .ok_or_else(|| {
        anyhow!(
            "No GitHub credentials available for workspace {}",
            context.workspace_id
        )
    })?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", credentials.password).parse()?,
    );
    headers.insert(
        reqwest::header::ACCEPT,
        "application/vnd.github+json".parse()?,
    );
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);
    headers.insert(reqwest::header::USER_AGENT, "codetether-pr-helper".parse()?);
    Ok(Client::builder().default_headers(headers).build()?)
}

async fn find_existing(client: &Client, context: &RepoContext) -> Result<Option<u64>> {
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

async fn create_pr(
    client: &Client,
    context: &RepoContext,
    args: &CreatePrArgs,
    body: &str,
) -> Result<PullRequestResponse> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        context.owner, context.repo
    );
    client
        .post(url)
        .json(&CreatePullRequestRequest {
            title: &args.title,
            head: &context.head_branch,
            base: &args.base,
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

async fn update_pr(
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
