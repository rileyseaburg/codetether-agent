//! GitHub repository env or trailer.
use super::*;
pub(super) fn env_or_trailer(env_key: &str, repo_root: &Path, trailer: &str) -> Option<String> {
    std::env::var(env_key)
        .ok()
        .or_else(|| commit_trailer(repo_root, trailer).ok().flatten())
}
