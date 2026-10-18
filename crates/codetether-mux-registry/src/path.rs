//! Global mux registry paths and safe name validation.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use directories::ProjectDirs;

/// Validate a bounded ASCII mux registry or session name.
/// # Arguments
/// `name` is the proposed name.
/// # Returns
/// Unit when it uses one to 64 permitted letters, digits, hyphens or underscores.
/// # Errors
/// Returns an error for empty, oversized or unsupported names.
/// # Examples
/// ```rust
/// assert!(codetether_mux_registry::validate_name("work_1").is_ok());
/// assert!(codetether_mux_registry::validate_name("../work").is_err());
/// ```
pub fn validate_name(name: &str) -> Result<()> {
    let valid = !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'));
    if !valid {
        bail!("mux name must use 1-64 letters, numbers, '-' or '_'");
    }
    Ok(())
}

pub(super) fn root() -> Result<PathBuf> {
    if let Ok(explicit) = std::env::var("CODETETHER_DATA_DIR")
        && !explicit.trim().is_empty()
    {
        return Ok(PathBuf::from(explicit).join("mux"));
    }
    ProjectDirs::from("ai", "codetether", "codetether-agent")
        .map(|dirs| dirs.data_dir().join("mux"))
        .context("could not determine mux registry directory")
}

pub(super) fn record(name: &str) -> Result<PathBuf> {
    validate_name(name)?;
    Ok(root()?.join(format!("{name}.json")))
}
