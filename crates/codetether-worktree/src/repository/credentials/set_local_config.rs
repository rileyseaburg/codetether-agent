//! Credential configuration operation: set local config.
use super::*;

/// Sets a local Git configuration value when the value is present.
///
/// Empty or missing values are ignored so optional settings stay ergonomic.
///
/// # Examples
///
/// ```ignore
/// set_local_config(repo_path, "codetether.githubAppId", Some("123"))?;
/// ```
pub fn set_local_config(repo_path: &Path, key: &str, value: Option<&str>) -> Result<()> {
    value
        .filter(|value| !value.trim().is_empty())
        .map_or(Ok(()), |value| {
            run_git_command(repo_path, &["config", "--local", key, value])
        })
}
