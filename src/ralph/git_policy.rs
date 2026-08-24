//! Sandboxed Git process adapters for approved Ralph orchestration.

use anyhow::{Result, bail};
use std::path::Path;
use std::process::Output;

pub(crate) fn prepare_commit(cwd: &Path) -> Result<()> {
    let args = vec!["add".to_string(), "-A".to_string()];
    let output = crate::tool::git::process::output_blocking(cwd, &args, &[], true)?;
    if !output.status.success() {
        bail!(
            "git add failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub(crate) fn checkout(cwd: &Path, branch: &str) -> Result<Output> {
    let args = vec!["checkout".to_string(), branch.to_string()];
    crate::tool::git::process::output_blocking(cwd, &args, &[], true)
}

pub(crate) fn checkout_new(cwd: &Path, branch: &str) -> Result<Output> {
    let args = vec!["checkout".to_string(), "-b".to_string(), branch.to_string()];
    crate::tool::git::process::output_blocking(cwd, &args, &[], true)
}
