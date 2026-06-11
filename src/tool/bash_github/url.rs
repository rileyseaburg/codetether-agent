//! GitHub auth loader for direct web requests.

use super::GitHubCommandAuth;
use super::config::resolve_gh_config_dir;
use super::credentials::load_github_password;
use super::repo_context::load_repo_context;
use anyhow::Result;
use reqwest::Url;

pub async fn load_github_url_auth(
    url: &Url,
    cwd: Option<&str>,
) -> Result<Option<GitHubCommandAuth>> {
    let Some(repo) = repo_path(url) else {
        return Ok(None);
    };
    let Some((workspace_id, host, path)) = load_repo_context(cwd).await else {
        return Ok(None);
    };
    if canonical(&path) != repo {
        return Ok(None);
    }
    let password = load_github_password(&workspace_id, host, path).await?;
    Ok(password.map(|password| GitHubCommandAuth::new(password, resolve_gh_config_dir())))
}

fn repo_path(url: &Url) -> Option<String> {
    let host = url.host_str()?.to_ascii_lowercase();
    let mut segments = url.path_segments()?;
    match host.as_str() {
        "api.github.com" => {
            (segments.next()? == "repos").then_some(())?;
            owner_repo(segments.next()?, segments.next()?)
        }
        "github.com" => owner_repo(segments.next()?, segments.next()?),
        _ => None,
    }
}

fn owner_repo(owner: &str, repo: &str) -> Option<String> {
    (!owner.is_empty() && !repo.is_empty()).then(|| canonical(&format!("{owner}/{repo}")))
}

fn canonical(path: &str) -> String {
    path.trim().trim_end_matches(".git").to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::repo_path;

    #[test]
    fn parses_github_api_repo_url() {
        let url = reqwest::Url::parse("https://api.github.com/repos/Owner/Repo/pulls/1").unwrap();
        assert_eq!(repo_path(&url).as_deref(), Some("owner/repo"));
    }
}
