//! Stable per-session location pointers.
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
mod path;
mod target;
use path::entry;
use target::valid_target;

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
    if std::fs::read_to_string(&entry).is_ok_and(|value| value == target.to_string_lossy()) {
        return Ok(());
    }
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
