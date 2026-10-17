//! GitHub repository git config.
use super::*;
pub(super) fn git_config(repo_root: &Path, key: &str) -> Result<Option<String>> {
    match git_stdout(Some(repo_root), ["config", "--local", "--get", key]) {
        Ok(value) if !value.is_empty() => Ok(Some(value)),
        Ok(_) => Ok(None),
        Err(_) => Ok(None),
    }
}
