//! Initialize benchmark Git identity and the initial scaffold commit.
use std::{path::Path, process::Command};
pub(super) fn initialize_git(workspace: &Path) -> anyhow::Result<()> {
    // git init + initial commit so Ralph can create branches
    let git_init = Command::new("git")
        .args(["init"])
        .current_dir(workspace)
        .output()?;
    if !git_init.status.success() {
        anyhow::bail!(
            "git init failed: {}",
            String::from_utf8_lossy(&git_init.stderr)
        );
    }

    // Set git user for commits (required for CI environments)
    let _ = Command::new("git")
        .args(["config", "user.email", "bench@codetether.run"])
        .current_dir(workspace)
        .output();
    let _ = Command::new("git")
        .args(["config", "user.name", "CodeTether Benchmark"])
        .current_dir(workspace)
        .output();

    // Initial commit with scaffolded project
    let _ = Command::new("git")
        .args(["add", "-A"])
        .current_dir(workspace)
        .output();
    let _ = Command::new("git")
        .args(["commit", "-m", "initial scaffold for benchmark"])
        .current_dir(workspace)
        .output();

    Ok(())
}
