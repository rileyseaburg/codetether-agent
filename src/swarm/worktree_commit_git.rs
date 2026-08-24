//! Sandboxed Git adapter for swarm worktree commits.

use anyhow::{Context, Result};
use std::path::Path;

pub(super) async fn run(path: &Path, args: &[&str]) -> Result<std::process::Output> {
    let owned = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    crate::tool::git::process::output(
        path,
        &owned,
        &[],
        args.first().is_some_and(|arg| *arg == "add"),
    )
    .await
    .with_context(|| format!("failed to run git {}", args.join(" ")))
}
