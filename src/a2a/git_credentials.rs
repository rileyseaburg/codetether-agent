use anyhow::{Context, Result, anyhow};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Default)]
pub struct GitCredentialQuery {
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GitCredentialMaterial {
    pub username: String,
    pub password: String,
    pub expires_at: Option<String>,
    pub token_type: String,
    pub host: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize)]
struct GitCredentialRequestBody<'a> {
    operation: &'a str,
    protocol: Option<&'a str>,
    host: Option<&'a str>,
    path: Option<&'a str>,
}

pub async fn request_git_credentials(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: Option<&str>,
    workspace_id: &str,
    operation: &str,
    query: &GitCredentialQuery,
) -> Result<Option<GitCredentialMaterial>> {
    let mut req = client.post(format!(
        "{}/v1/agent/workspaces/{}/git/credentials",
        server.trim_end_matches('/'),
        workspace_id
    ));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }
    if let Some(worker_id) = worker_id
        && !worker_id.trim().is_empty()
    {
        req = req.header("X-Worker-ID", worker_id);
    }

    let response = req
        .json(&GitCredentialRequestBody {
            operation,
            protocol: query.protocol.as_deref(),
            host: query.host.as_deref(),
            path: query.path.as_deref(),
        })
        .send()
        .await
        .context("Failed to request Git credentials from server")?;

    match response.status() {
        StatusCode::OK => Ok(Some(
            response
                .json::<GitCredentialMaterial>()
                .await
                .context("Failed to decode Git credential response")?,
        )),
        StatusCode::NOT_FOUND => Ok(None),
        status => {
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!(
                "Git credential request failed with {}: {}",
                status,
                body
            ))
        }
    }
}

pub async fn run_git_credential_helper(args: &crate::cli::GitCredentialHelperArgs) -> Result<()> {
    let operation = args.operation.as_deref().unwrap_or("get").trim();
    if matches!(operation, "store" | "erase") {
        return Ok(());
    }

    let query = read_git_credential_query_from_stdin()?;
    let server = args
        .server
        .clone()
        .or_else(|| std::env::var("CODETETHER_SERVER").ok())
        .ok_or_else(|| anyhow!("CODETETHER_SERVER is not set for Git credential helper"))?;
    let token = args
        .token
        .clone()
        .or_else(|| std::env::var("CODETETHER_TOKEN").ok());
    let worker_id = args
        .worker_id
        .clone()
        .or_else(|| std::env::var("CODETETHER_WORKER_ID").ok());

    let client = Client::new();
    let credentials = request_git_credentials(
        &client,
        &server,
        &token,
        worker_id.as_deref(),
        &args.workspace_id,
        operation,
        &query,
    )
    .await?;

    if let Some(creds) = credentials {
        if should_delegate_to_gh_cli(&query, &creds)
            && emit_credentials_via_gh_cli(&query, &creds).is_ok()
        {
            return Ok(());
        }

        print_git_credentials(&creds);
    }

    Ok(())
}

fn should_delegate_to_gh_cli(
    query: &GitCredentialQuery,
    credentials: &GitCredentialMaterial,
) -> bool {
    let host = query
        .host
        .as_deref()
        .or(credentials.host.as_deref())
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    host == "github.com"
}

fn emit_credentials_via_gh_cli(
    query: &GitCredentialQuery,
    credentials: &GitCredentialMaterial,
) -> Result<()> {
    let mut child = Command::new("gh")
        .args(["auth", "git-credential", "get"])
        .env("GH_TOKEN", &credentials.password)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn gh auth git-credential")?;

    let request = render_gh_credential_query(query, credentials);
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(request.as_bytes())
            .context("Failed to write Git credential request to gh")?;
    }

    let output = child
        .wait_with_output()
        .context("Failed to read gh auth git-credential output")?;
    if output.status.success() {
        return Ok(());
    }

    Err(anyhow!(
        "gh auth git-credential failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

fn render_gh_credential_query(
    query: &GitCredentialQuery,
    credentials: &GitCredentialMaterial,
) -> String {
    let mut request = String::new();
    let protocol = query.protocol.as_deref().unwrap_or("https");
    request.push_str(&format!("protocol={protocol}\n"));

    if let Some(host) = query.host.as_deref().or(credentials.host.as_deref()) {
        request.push_str(&format!("host={}\n", host.trim()));
    }
    if let Some(path) = query.path.as_deref().or(credentials.path.as_deref()) {
        request.push_str(&format!("path={}\n", path.trim()));
    }
    request.push('\n');
    request
}

fn print_git_credentials(credentials: &GitCredentialMaterial) {
    println!("username={}", credentials.username);
    println!("password={}", credentials.password);
    println!();
}

pub fn configure_repo_git_auth(repo_path: &Path, workspace_id: &str) -> Result<PathBuf> {
    let helper_path = repo_path.join(".codetether-git-credential-helper");
    write_git_credential_helper_script(&helper_path, workspace_id)?;
    run_git_command(
        repo_path,
        [
            "config",
            "--local",
            "credential.helper",
            helper_path
                .to_str()
                .ok_or_else(|| anyhow!("Helper path is not valid UTF-8"))?,
        ],
    )?;
    run_git_command(
        repo_path,
        ["config", "--local", "credential.useHttpPath", "true"],
    )?;
    run_git_command(
        repo_path,
        ["config", "--local", "codetether.workspaceId", workspace_id],
    )?;
    Ok(helper_path)
}

pub fn write_git_credential_helper_script(script_path: &Path, workspace_id: &str) -> Result<()> {
    if let Some(parent) = script_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create helper script directory {}",
                parent.display()
            )
        })?;
    }

    let current_exe = std::env::current_exe()
        .context("Failed to determine current CodeTether binary path for credential helper")?;
    let script = format!(
        "#!/bin/sh\nexec {} git-credential-helper --workspace-id {} \"$@\"\n",
        shell_single_quote(current_exe.to_string_lossy().as_ref()),
        shell_single_quote(workspace_id),
    );
    fs::write(script_path, script).with_context(|| {
        format!(
            "Failed to write Git credential helper script {}",
            script_path.display()
        )
    })?;

    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(script_path)
            .with_context(|| {
                format!(
                    "Failed to stat Git credential helper script {}",
                    script_path.display()
                )
            })?
            .permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(script_path, permissions).with_context(|| {
            format!(
                "Failed to mark Git credential helper script executable {}",
                script_path.display()
            )
        })?;
    }

    Ok(())
}

fn run_git_command<I, S>(repo_path: &Path, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args_vec: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(args_vec.iter().map(String::as_str))
        .output()
        .with_context(|| format!("Failed to run git command in {}", repo_path.display()))?;

    if output.status.success() {
        return Ok(());
    }

    Err(anyhow!(
        "Git command failed in {}: {}",
        repo_path.display(),
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

fn read_git_credential_query_from_stdin() -> Result<GitCredentialQuery> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read Git credential request from stdin")?;

    let mut query = GitCredentialQuery::default();
    for line in input.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            match key.trim() {
                "protocol" => query.protocol = Some(value.to_string()),
                "host" => query.host = Some(value.to_string()),
                "path" => query.path = Some(value.to_string()),
                _ => {}
            }
        }
    }

    Ok(query)
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests {
    use super::{
        GitCredentialMaterial, GitCredentialQuery, render_gh_credential_query,
        should_delegate_to_gh_cli,
    };

    fn sample_credentials() -> GitCredentialMaterial {
        GitCredentialMaterial {
            username: "x-access-token".to_string(),
            password: "secret".to_string(),
            expires_at: None,
            token_type: "github_app".to_string(),
            host: Some("github.com".to_string()),
            path: Some("owner/repo.git".to_string()),
        }
    }

    #[test]
    fn delegates_to_gh_for_github_host() {
        let mut query = GitCredentialQuery::default();
        query.host = Some("github.com".to_string());

        assert!(should_delegate_to_gh_cli(&query, &sample_credentials()));
    }

    #[test]
    fn skips_gh_for_non_github_host() {
        let mut query = GitCredentialQuery::default();
        query.host = Some("gitlab.com".to_string());

        assert!(!should_delegate_to_gh_cli(&query, &sample_credentials()));
    }

    #[test]
    fn renders_git_credential_payload_for_gh() {
        let mut query = GitCredentialQuery::default();
        query.protocol = Some("https".to_string());

        let rendered = render_gh_credential_query(&query, &sample_credentials());
        assert!(rendered.contains("protocol=https\n"));
        assert!(rendered.contains("host=github.com\n"));
        assert!(rendered.contains("path=owner/repo.git\n"));
        assert!(rendered.ends_with("\n\n"));
    }
}
