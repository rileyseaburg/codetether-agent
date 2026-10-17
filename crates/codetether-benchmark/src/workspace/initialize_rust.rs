//! Initialize the Rust project and required serialization dependency.
use std::{path::Path, process::Command};
use tracing::warn;
pub(super) fn initialize_rust(workspace: &Path, project: &str) -> anyhow::Result<()> {
    // cargo init --name <project>
    let cargo_init = Command::new("cargo")
        .args(["init", "--name", project])
        .current_dir(workspace)
        .output()?;
    if !cargo_init.status.success() {
        anyhow::bail!(
            "cargo init failed: {}",
            String::from_utf8_lossy(&cargo_init.stderr)
        );
    }

    // Add common benchmark dependencies to Cargo.toml
    // which crates to add depends on the PRD
    let cargo_add_args: Vec<&str> = vec!["add", "serde", "--features", "derive"];
    let cargo_add = Command::new("cargo")
        .args(&cargo_add_args)
        .current_dir(workspace)
        .output()?;
    if !cargo_add.status.success() {
        warn!(
            "cargo add serde failed (non-fatal): {}",
            String::from_utf8_lossy(&cargo_add.stderr)
        );
    }

    Ok(())
}
