//! Cargo workspace stub injection for worktree isolation.
//!
//! Ensures each worktree's `Cargo.toml` declares `[workspace]` so the
//! worktree is its own Cargo workspace root and is not absorbed by an
//! ancestor workspace's compilation graph or `target/` cache. The local
//! edit is marked `--skip-worktree` so it cannot leak into a commit that
//! gets merged back into the parent branch.

use anyhow::{Context, Result};
use std::path::Path;

const STUB_HEADER: &str = "[workspace]\n";

/// Idempotently prepend `[workspace]` to the worktree's Cargo.toml.
///
/// Repos without a Cargo.toml (or that already declare `[workspace]`)
/// are left untouched. After modifying the file we mark it
/// `--skip-worktree` in the worktree's index so the change cannot be
/// staged or merged back into the parent branch.
pub fn inject(worktree_path: &Path) -> Result<()> {
    let cargo_toml = worktree_path.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Ok(());
    }
    let original = std::fs::read_to_string(&cargo_toml)
        .with_context(|| format!("Failed to read {}", cargo_toml.display()))?;
    if original.contains("[workspace]") {
        return Ok(());
    }
    let updated = format!("{STUB_HEADER}{original}");
    std::fs::write(&cargo_toml, updated)
        .with_context(|| format!("Failed to write {}", cargo_toml.display()))?;
    let _ = std::process::Command::new("git")
        .args(["update-index", "--skip-worktree", "Cargo.toml"])
        .current_dir(worktree_path)
        .output();
    tracing::info!(
        worktree = %worktree_path.display(),
        "Injected [workspace] stub into worktree Cargo.toml"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::inject;

    #[test]
    fn skips_when_no_cargo_toml() {
        let dir = tempfile::tempdir().expect("tempdir");
        inject(dir.path()).expect("non-Rust worktree should be a silent no-op");
    }

    #[test]
    fn idempotent_when_workspace_already_present() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cargo_toml = dir.path().join("Cargo.toml");
        std::fs::write(&cargo_toml, "[workspace]\n[package]\nname = \"x\"\n").expect("write");
        inject(dir.path()).expect("idempotent");
        let contents = std::fs::read_to_string(&cargo_toml).expect("read");
        let occurrences = contents.matches("[workspace]").count();
        assert_eq!(occurrences, 1, "stub must not be duplicated");
    }
}
