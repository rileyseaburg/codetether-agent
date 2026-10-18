//! Cross-platform AWS config/cache path resolution.
//!
//! AWS CLI honours `AWS_CONFIG_FILE` / `AWS_SHARED_CREDENTIALS_FILE` and
//! otherwise stores config under the user home directory (`~/.aws` on Unix,
//! `%USERPROFILE%\.aws` on Windows). Resolving via [`directories`] avoids the
//! Unix-only `HOME` assumption that breaks the Windows `connect` flow.

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Path to `~/.aws/config`, honouring `AWS_CONFIG_FILE` when set.
pub(super) fn config_path() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("AWS_CONFIG_FILE") {
        return Ok(PathBuf::from(path));
    }
    Ok(aws_dir()?.join("config"))
}

/// Path to the `~/.aws/sso/cache` directory.
pub(super) fn sso_cache_dir() -> Result<PathBuf> {
    Ok(aws_dir()?.join("sso").join("cache"))
}

/// Resolve the `.aws` directory under the platform home directory.
fn aws_dir() -> Result<PathBuf> {
    let home = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        .context("could not determine the user home directory")?;
    Ok(home.join(".aws"))
}
