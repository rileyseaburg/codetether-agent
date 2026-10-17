//! GitHub repository commit trailer.
use super::*;
pub(super) fn commit_trailer(repo_root: &Path, label: &str) -> Result<Option<String>> {
    let message = git_stdout(Some(repo_root), ["log", "-1", "--format=%B"])?;
    Ok(message.lines().find_map(|line| {
        line.strip_prefix(&format!("{label}:"))
            .map(|value| value.trim().to_string())
    }))
}
