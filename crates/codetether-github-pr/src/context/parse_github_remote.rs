//! GitHub repository parse github remote.
use super::*;
pub(super) fn parse_github_remote(remote_url: &str) -> Result<(String, String)> {
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
