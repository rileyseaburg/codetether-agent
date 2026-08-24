//! Sandboxed Git reads used to preview undo operations.

use anyhow::Result;
use std::path::Path;

pub(super) fn repository(cwd: &Path) -> Result<std::process::Output> {
    crate::tool::git::process::output_blocking_refs(cwd, &["rev-parse", "--git-dir"], false)
}

pub(super) fn log(cwd: &Path, steps: usize) -> Result<std::process::Output> {
    let count = steps.to_string();
    crate::tool::git::process::output_blocking_refs(
        cwd,
        &["log", "--oneline", "--max-count", &count],
        false,
    )
}

pub(super) fn diff(cwd: &Path, steps: usize) -> Result<std::process::Output> {
    let revision = format!("HEAD~{steps}");
    crate::tool::git::process::output_blocking_refs(cwd, &["diff", &revision, "--name-only"], false)
}
