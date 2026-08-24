//! Borrowed-argument adapters for sandboxed Git execution.

use anyhow::Result;
use std::path::Path;

pub(crate) async fn output_refs(
    cwd: &Path,
    args: &[&str],
    mutating: bool,
) -> Result<std::process::Output> {
    let args = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    super::output(cwd, &args, &[], mutating).await
}

pub(crate) fn output_blocking_refs(
    cwd: &Path,
    args: &[&str],
    mutating: bool,
) -> Result<std::process::Output> {
    let args = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    super::output_blocking(cwd, &args, &[], mutating)
}
