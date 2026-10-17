//! File I/O helpers for patch application.

use anyhow::{Context, Result};
use std::path::Path;

/// Read an existing file, or return empty content for a new file target.
pub(super) fn read_existing(path: &Path, file: &str) -> Result<String> {
    if path.exists() {
        std::fs::read_to_string(path).context(format!("Failed to read {file}"))
    } else {
        Ok(String::new())
    }
}

/// Persist patched content, creating parent directories as needed.
pub(super) fn write_updated(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(std::fs::write(path, content)?)
}
