//! Cargo workspace stub injection for worktree isolation.
//!
//! Ensures each worktree's `Cargo.toml` declares `[workspace]` so the
//! worktree is its own Cargo workspace root and is not absorbed by an
//! ancestor workspace's compilation graph or `target/` cache. The local
//! edit is marked `--skip-worktree` so it cannot leak into a commit that
//! gets merged back into the parent branch.

use anyhow::{Context, Result};
use std::path::Path;

mod target_config;

const STUB_HEADER: &str = "[workspace]\n";

/// Idempotently prepend `[workspace]` to the worktree's Cargo.toml.
///
/// Repos without a Cargo.toml are left untouched. Rust worktrees also get a
/// local Cargo `target-dir` config under the manager's artifact directory.
/// Local edits are marked `--skip-worktree` so they cannot be committed.
pub fn inject(worktree_path: &Path, artifact_base: &Path) -> Result<()> {
    let cargo_toml = worktree_path.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Ok(());
    }
    let original = std::fs::read_to_string(&cargo_toml)
        .with_context(|| format!("Failed to read {}", cargo_toml.display()))?;
    if !original.contains("[workspace]") {
        let updated = format!("{STUB_HEADER}{original}");
        std::fs::write(&cargo_toml, updated)
            .with_context(|| format!("Failed to write {}", cargo_toml.display()))?;
        mark_skip_worktree(worktree_path, "Cargo.toml");
        tracing::info!(
            worktree = %worktree_path.display(),
            "Injected [workspace] stub into worktree Cargo.toml"
        );
    }
    target_config::inject(worktree_path, artifact_base)?;
    Ok(())
}

fn mark_skip_worktree(worktree_path: &Path, file: &str) {
    let _ = std::process::Command::new("git")
        .args(["add", "-N", file])
        .current_dir(worktree_path)
        .output();
    let _ = std::process::Command::new("git")
        .args(["update-index", "--skip-worktree", file])
        .current_dir(worktree_path)
        .output();
}

#[cfg(test)]
mod tests;
