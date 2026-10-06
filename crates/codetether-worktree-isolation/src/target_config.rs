use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[path = "target_config_text.rs"]
mod text;

pub fn inject(worktree_path: &Path, artifact_base: &Path) -> Result<()> {
    let config = worktree_path.join(".cargo/config.toml");
    let original = std::fs::read_to_string(&config).unwrap_or_default();
    if original.lines().any(has_target_dir) {
        return Ok(());
    }
    let target = artifact_target(worktree_path, artifact_base);
    let line = format!("target-dir = {}", text::quoted(&target));
    let updated = text::with_target_dir(&original, &line);
    if let Some(parent) = config.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    std::fs::write(&config, updated)
        .with_context(|| format!("Failed to write {}", config.display()))?;
    super::mark_skip_worktree(worktree_path, ".cargo/config.toml");
    Ok(())
}

fn artifact_target(worktree_path: &Path, artifact_base: &Path) -> PathBuf {
    let name = worktree_path.file_name().unwrap_or_default();
    artifact_base.join(".targets").join(name)
}

fn has_target_dir(line: &str) -> bool {
    line.trim_start().starts_with("target-dir")
}
