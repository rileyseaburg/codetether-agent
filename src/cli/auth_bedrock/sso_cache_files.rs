//! Filesystem discovery for AWS SSO cache JSON files.

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Return candidate `~/.aws/sso/cache/*.json` files.
pub(super) fn json_files() -> Result<Vec<PathBuf>> {
    let home = std::env::var("HOME").context("HOME is not set")?;
    let dir = PathBuf::from(home).join(".aws/sso/cache");
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error).context("Failed to read ~/.aws/sso/cache"),
    };
    Ok(entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect())
}
