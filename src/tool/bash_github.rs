use crate::a2a::git_credentials::{GitCredentialQuery, request_git_credentials};
use anyhow::Result;
use reqwest::Client;
use tokio::process::Command;

pub struct GitHubCommandAuth {
    pub env: Vec<(&'static str, String)>,
    pub redactions: Vec<String>,
}

pub async fn load_github_command_auth(
    command: &str,
    cwd: Option<&str>,
) -> Result<Option<GitHubCommandAuth>> {
    if !needs_github_auth(command) {
        return Ok(None);
    }
    let Some(repo_root) = git_stdout(cwd, ["rev-parse", "--show-toplevel"]).await else {
        return Ok(None);
    };
    let Some(workspace_id) = git_stdout(
        Some(repo_root.as_str()),
        ["config", "--local", "--get", "codetether.workspaceId"],
    )
    .await
    else {
        return Ok(None);
    };
    let Some(remote_url) =
        git_stdout(Some(repo_root.as_str()), ["remote", "get-url", "origin"]).await
    else {
        return Ok(None);
    };
    let Some((host, path)) = parse_https_remote(&remote_url) else {
        return Ok(None);
    };
    if host != "github.com" {
        return Ok(None);
    }
    let Ok(server) = std::env::var("CODETETHER_SERVER") else {
        return Ok(None);
    };
    let token = std::env::var("CODETETHER_TOKEN").ok();
    let worker_id = std::env::var("CODETETHER_WORKER_ID").ok();
    let credentials = request_git_credentials(
        &Client::new(),
        &server,
        &token,
        worker_id.as_deref(),
        workspace_id.as_str(),
        "get",
        &GitCredentialQuery {
            protocol: Some("https".to_string()),
            host: Some(host),
            path: Some(path),
        },
    )
    .await?;
    let Some(credentials) = credentials else {
        return Ok(None);
    };
    Ok(Some(GitHubCommandAuth {
        env: vec![
            ("GH_TOKEN", credentials.password.clone()),
            ("GITHUB_TOKEN", credentials.password.clone()),
            ("GH_HOST", "github.com".to_string()),
            ("GH_PROMPT_DISABLED", "1".to_string()),
        ],
        redactions: vec![credentials.password],
    }))
}

async fn git_stdout<const N: usize>(cwd: Option<&str>, args: [&str; N]) -> Option<String> {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = command.output().await.ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn needs_github_auth(command: &str) -> bool {
    let lower = command.to_ascii_lowercase();
    lower.starts_with("gh ")
        || lower.contains("\ngh ")
        || lower.contains(" gh ")
        || lower.contains("api.github.com")
        || lower.contains("github.com/repos/")
}

fn parse_https_remote(remote_url: &str) -> Option<(String, String)> {
    let trimmed = remote_url.trim().strip_prefix("https://")?;
    let (host, path) = trimmed.split_once('/')?;
    Some((host.trim().to_ascii_lowercase(), path.trim().to_string()))
}
