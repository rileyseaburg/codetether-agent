use anyhow::{Context, Result};
use std::path::Path;

#[cfg(unix)]
pub(super) fn create(source: &Path, destination: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, destination)
        .with_context(|| format!("failed to link {}", destination.display()))
}

#[cfg(windows)]
pub(super) fn create(source: &Path, destination: &Path) -> Result<()> {
    std::os::windows::fs::symlink_dir(source, destination)
        .with_context(|| format!("failed to link {}", destination.display()))
}
