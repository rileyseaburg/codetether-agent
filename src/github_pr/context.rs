use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct RepoContext {
    pub repo_root: PathBuf,
    pub owner: String,
    pub repo: String,
    pub head_branch: String,
    pub head_sha: String,
    pub workspace_id: String,
    pub github_installation_id: Option<String>,
    pub github_app_id: Option<String>,
    pub signature: SignatureContext,
}

#[derive(Clone, Debug)]
pub struct SignatureContext {
    pub agent_identity_id: Option<String>,
    pub model: Option<String>,
    pub tenant_id: Option<String>,
    pub worker_id: Option<String>,
    pub run_id: Option<String>,
    pub attempt_id: Option<String>,
    pub session_id: Option<String>,
    pub provenance_signature: Option<String>,
    pub github_installation_id: Option<String>,
    pub github_app_id: Option<String>,
    pub head_sha: String,
    pub workspace_id: String,
}

impl SignatureContext {
    #[cfg(test)]
    pub fn sample() -> Self {
        Self {
            agent_identity_id: Some("user:test".to_string()),
            model: Some("zai/glm-5".to_string()),
            tenant_id: None,
            worker_id: None,
            run_id: None,
            attempt_id: None,
            session_id: None,
            provenance_signature: Some("sig".to_string()),
            github_installation_id: None,
            github_app_id: None,
            head_sha: "abc123".to_string(),
            workspace_id: "ws-1".to_string(),
        }
    }
}

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

fn env_or_trailer(env_key: &str, repo_root: &Path, trailer: &str) -> Option<String> {
    std::env::var(env_key)
        .ok()
        .or_else(|| commit_trailer(repo_root, trailer).ok().flatten())
}

fn git_stdout<const N: usize>(cwd: Option<&Path>, args: [&str; N]) -> Result<String> {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = command.output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "{}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_config(repo_root: &Path, key: &str) -> Result<Option<String>> {
    match git_stdout(Some(repo_root), ["config", "--local", "--get", key]) {
        Ok(value) if !value.is_empty() => Ok(Some(value)),
        Ok(_) => Ok(None),
        Err(_) => Ok(None),
    }
}

fn commit_trailer(repo_root: &Path, label: &str) -> Result<Option<String>> {
    let message = git_stdout(Some(repo_root), ["log", "-1", "--format=%B"])?;
    Ok(message.lines().find_map(|line| {
        line.strip_prefix(&format!("{label}:"))
            .map(|value| value.trim().to_string())
    }))
}

fn parse_github_remote(remote_url: &str) -> Result<(String, String)> {
    let trimmed = remote_url.trim().trim_end_matches(".git");
    let path = if let Some(path) = trimmed.strip_prefix("https://github.com/") {
        path
    } else if let Some(path) = trimmed.strip_prefix("ssh://git@github.com/") {
        path
    } else if let Some(path) = trimmed.strip_prefix("git@github.com:") {
        path
    } else {
        return Err(anyhow!("Unsupported GitHub remote URL: {trimmed}"));
    };
    let (owner, repo) = path
        .split_once('/')
        .ok_or_else(|| anyhow!("Invalid GitHub remote path: {path}"))?;
    Ok((owner.to_string(), repo.to_string()))
}

#[cfg(test)]
mod tests {
    use super::parse_github_remote;

    #[test]
    fn parses_https_remote() {
        let (owner, repo) = parse_github_remote("https://github.com/a/b.git").unwrap();
        assert_eq!((owner.as_str(), repo.as_str()), ("a", "b"));
    }

    #[test]
    fn parses_ssh_remote() {
        let (owner, repo) = parse_github_remote("git@github.com:a/b.git").unwrap();
        assert_eq!((owner.as_str(), repo.as_str()), ("a", "b"));
    }
}
