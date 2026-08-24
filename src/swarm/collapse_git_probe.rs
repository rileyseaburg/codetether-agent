//! Non-building Git health check for speculative branch probes.

use anyhow::{Context, Result};
use std::path::Path;

pub(super) fn static_clean(worktree: &Path) -> Result<bool> {
    let output =
        crate::tool::git::process::output_blocking_refs(worktree, &["diff", "--check"], false)
            .context("Failed to run static Git branch check")?;
    Ok(output.status.success())
}
