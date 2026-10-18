//! Git subprocess errors for swarm worktree preparation.
use anyhow::{Context, Result};
use tokio::process::Command;
pub(super) async fn run(path: &std::path::Path, args: &[&str]) -> Result<std::process::Output> {
    Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .await
        .with_context(|| format!("failed to run git {}", args.join(" ")))
}
