//! Stable per-session pointers used when ownership moves between worktrees.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;

pub(super) fn read(id: &str) -> Result<Option<PathBuf>> {
    let entry = entry(id)?;
    let Ok(value) = std::fs::read_to_string(entry) else {
        return Ok(None);
    };
    let path = PathBuf::from(value.trim());
    Ok(valid_target(id, &path).then_some(path))
}

pub(super) fn write(id: &str, target: &Path) -> Result<()> {
    let entry = entry(id)?;
    if let Some(parent) = entry.parent() {
        std::fs::create_dir_all(parent).context("create session location directory")?;
    }
    std::fs::write(entry, target.to_string_lossy().as_bytes()).context("write session location")
}

pub(super) fn remove(id: &str) -> Result<()> {
    match std::fs::remove_file(entry(id)?) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).context("remove session location"),
    }
}

fn entry(id: &str) -> Result<PathBuf> {
    let root = if let Some(path) = explicit_root() {
        path
    } else {
        ProjectDirs::from("ai", "codetether", "codetether-agent")
            .context("resolve global CodeTether data directory")?
            .data_dir()
            .to_path_buf()
    };
    Ok(root.join("session-locations").join(format!("{id}.path")))
}

fn explicit_root() -> Option<PathBuf> {
    std::env::var("CODETETHER_DATA_DIR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
}

fn valid_target(id: &str, path: &Path) -> bool {
    path.is_absolute()
        && path.is_file()
        && path.file_name().and_then(|name| name.to_str()) == Some(&format!("{id}.json"))
}
