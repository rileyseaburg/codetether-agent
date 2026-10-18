//! Adapt CLI body and project arguments to the shared PR service.
use super::*;

pub async fn create_or_update(args: CreatePrArgs) -> Result<PullRequestResult> {
    let context = load_repo_context(project_root(args.project.as_ref()), args.head.as_deref())?;
    let body = build_body(&args, &context)?;
    let client = github_client(&context).await?;
    let spec = codetether_github_pr::http::PullRequestSpec {
        title: &args.title,
        base: &args.base,
        body: &body,
        draft: args.draft,
    };
    let pr = codetether_github_pr::http::create_or_update(&client, &context, &spec).await?;
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
